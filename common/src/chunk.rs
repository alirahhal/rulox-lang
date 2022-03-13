use crate::value::{Value, ValueArray};

#[derive(Debug, PartialEq)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpConstantLong,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
    OpPrint,
    OpPop,
    OpDefineGlobal,
    OpDefineGlobalLong,
    OpGetGlobal,
    OpGetGlobalLong,
    OpSetGlobal,
    OpSetGlobalLong,
    OpGetLocal,
    OpGetLocalLong,
    OpSetLocal,
    OpSetLocalLong,
    OpJumpIfFalse,
    OpJump,
    OpLoop,

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
        3 => Some(OpCode::OpAdd),
        4 => Some(OpCode::OpSubtract),
        5 => Some(OpCode::OpMultiply),
        6 => Some(OpCode::OpDivide),
        7 => Some(OpCode::OpNegate),
        8 => Some(OpCode::OpNil),
        9 => Some(OpCode::OpTrue),
        10 => Some(OpCode::OpFalse),
        11 => Some(OpCode::OpNot),
        12 => Some(OpCode::OpEqual),
        13 => Some(OpCode::OpGreater),
        14 => Some(OpCode::OpLess),
        15 => Some(OpCode::OpPrint),
        16 => Some(OpCode::OpPop),
        17 => Some(OpCode::OpDefineGlobal),
        18 => Some(OpCode::OpDefineGlobalLong),
        19 => Some(OpCode::OpGetGlobal),
        20 => Some(OpCode::OpGetGlobalLong),
        21 => Some(OpCode::OpSetGlobal),
        22 => Some(OpCode::OpSetGlobalLong),
        23 => Some(OpCode::OpGetLocal),
        24 => Some(OpCode::OpGetLocalLong),
        25 => Some(OpCode::OpSetLocal),
        26 => Some(OpCode::OpSetLocalLong),
        27 => Some(OpCode::OpJumpIfFalse),
        28 => Some(OpCode::OpJump),
        29 => Some(OpCode::OpLoop),

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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::common::opcode_from_u8;

//     #[test]
//     fn write_constant_appends_constant_operation() {
//         let mut chunk = Chunk::new();

//         chunk.write_constant(1 as f64, 1);

//         assert_eq!(
//             opcode_from_u8(chunk.code[chunk.code.len() - 2]).unwrap(),
//             OpCode::OpConstant,
//             "Write constant did not emit a constant operation, found: `{:?}`",
//             opcode_from_u8(chunk.code[chunk.code.len() - 2]).unwrap()
//         );
//     }

//     #[test]
//     fn write_constant_once() {
//         let mut chunk = Chunk::new();

//         chunk.write_constant(1 as f64, 1);

//         assert_eq!(
//             chunk.constants.values[chunk.code[chunk.code.len() - 1] as usize],
//             1 as f64,
//         );
//     }

//     #[test]
//     fn write_constant_twice() {
//         let mut chunk = Chunk::new();

//         chunk.write_constant(1 as f64, 1);
//         chunk.write_constant(2 as f64, 1);

//         assert_eq!(
//             chunk.constants.values[chunk.code[chunk.code.len() - 3] as usize],
//             1 as f64,
//         );
//         assert_eq!(
//             chunk.constants.values[chunk.code[chunk.code.len() - 1] as usize],
//             2 as f64,
//         );
//     }

//     #[test]
//     fn write_constant_appends_long_constant_operation() {
//         let mut chunk = Chunk::new();

//         let mut i = 0;
//         while i < 257 {
//             chunk.write_constant(i as f64, 1);
//             assert_eq!(
//                 opcode_from_u8(chunk.code[chunk.code.len() - 2]).unwrap(),
//                 OpCode::OpConstant,
//                 "Write constant did not emit a constant operation, found: `{:?}`",
//                 opcode_from_u8(chunk.code[chunk.code.len() - 2]).unwrap()
//             );
//             i = i + 1;
//         }

//         assert_eq!(
//             opcode_from_u8(chunk.code[chunk.code.len() - 4]).unwrap(),
//             OpCode::OpConstantLong,
//             "Write constant did not emit a long constant operation, found: `{:?}`",
//             opcode_from_u8(chunk.code[chunk.code.len() - 4]).unwrap()
//         );
//     }
// }
