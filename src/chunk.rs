use crate::value::{Value, ValueArray};

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpConstantLong,
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

        if index < 1 {
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
