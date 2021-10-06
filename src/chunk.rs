use crate::value::{Value, ValueArray};

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpConstantLong,
    Unknown,
}

impl Default for OpCode {
    fn default() -> Self {
        OpCode::Unknown
    }
}

pub fn opcode_from_u8(n: u8) -> Option<OpCode> {
    match n {
        0 => Some(OpCode::OpReturn),
        1 => Some(OpCode::OpConstant),
        2 => Some(OpCode::OpConstantLong),
        _ => None,
    }
}

#[derive(Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    pub lines: Vec<i32>,
}

impl Chunk {
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
        return (self.constants.values.len() - 1) as i32;
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
    fn test_write_constant() {
        let mut chunk = Chunk {
            ..Default::default()
        };

        chunk.write_constant(1 as f64, 1);

        assert_eq!(
            opcode_from_u8(chunk.code[chunk.code.len() - 2]).unwrap(),
            OpCode::OpConstant
        );
    }

    #[test]
    fn test_write_long_constant() {
        let mut chunk = Chunk {
            ..Default::default()
        };

        let mut i = 0;
        while i < 257 {
            chunk.write_constant(i as f64, 1);
            i = i + 1;
        }

        assert_eq!(
            opcode_from_u8(chunk.code[chunk.code.len() - 4]).unwrap(),
            OpCode::OpConstantLong
        );
    }
}
