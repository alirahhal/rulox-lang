#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpConstantLong,
    OpAdd,
    OpSubstract,
    OpMultiply,
    OpDivide,
    OpNegate,
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
        4 => Some(OpCode::OpSubstract),
        5 => Some(OpCode::OpMultiply),
        6 => Some(OpCode::OpDivide),
        7 => Some(OpCode::OpNegate),
        _ => None,
    }
}
