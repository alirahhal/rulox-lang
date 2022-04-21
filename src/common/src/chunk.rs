use crate::{
    opcode::OpCode,
    value::{Value, ValueArray},
};

#[derive(Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    pub lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn write_chunk(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, value: Value, line: i32) {
        let index = self.add_constant(value);

        if index < 256 {
            self.write_chunk(OpCode::OpConstant as u8, line);
            self.write_chunk(index as u8, line);
        } else {
            self.write_chunk(OpCode::OpConstantLong as u8, line);
            self.write_chunk((index & 0xff) as u8, line);
            self.write_chunk(((index >> 8) & 0xff) as u8, line);
            self.write_chunk(((index >> 16) & 0xff) as u8, line);
        }
    }

    pub fn add_constant(&mut self, value: Value) -> i32 {
        self.constants.write_value_array(value);
        (self.constants.values.len() - 1) as i32
    }

    pub fn free_chunk(&mut self) {
        self.code.clear();
        self.lines.clear();
        self.constants.free_value_array();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_constant_appends_constant_operation() {
        let mut chunk = Chunk::new();

        chunk.write_constant(Value::Number(1), 1);

        assert_eq!(
            OpCode::try_from(chunk.code[chunk.code.len() - 2]),
            Ok(OpCode::OpConstant),
            "Write constant did not emit a constant operation, found: `{:?}`",
            OpCode::try_from(chunk.code[chunk.code.len() - 2]),
        );
    }

    #[test]
    fn write_constant_once() {
        let mut chunk = Chunk::new();

        chunk.write_constant(Value::Number(1), 1);

        assert_eq!(
            (chunk.constants.values[chunk.code[chunk.code.len() - 1] as usize]).as_number(),
            Value::Number(1).as_number(),
        );
    }

    #[test]
    fn write_constant_twice() {
        let mut chunk = Chunk::new();

        chunk.write_constant(Value::Number(1), 1);
        chunk.write_constant(Value::Number(2), 1);

        assert_eq!(
            (chunk.constants.values[chunk.code[chunk.code.len() - 3] as usize]).as_number(),
            Value::Number(1).as_number(),
        );
        assert_eq!(
            (chunk.constants.values[chunk.code[chunk.code.len() - 1] as usize]).as_number(),
            Value::Number(2).as_number(),
        );
    }

    #[test]
    fn write_constant_appends_long_constant_operation() {
        let mut chunk = Chunk::new();

        let mut i = 0;
        while i < 257 {
            chunk.write_constant(Value::Number(i), 1);
            assert_eq!(
                OpCode::try_from(chunk.code[chunk.code.len() - 2]),
                Ok(OpCode::OpConstant),
                "Write constant did not emit a constant operation, found: `{:?}`",
                OpCode::try_from(chunk.code[chunk.code.len() - 2]),
            );
            i = i + 1;
        }

        assert_eq!(
            OpCode::try_from(chunk.code[chunk.code.len() - 4]),
            Ok(OpCode::OpConstantLong),
            "Write constant did not emit a long constant operation, found: `{:?}`",
            OpCode::try_from(chunk.code[chunk.code.len() - 4]),
        );
    }
}
