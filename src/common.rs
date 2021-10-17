#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpConstantLong,
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
        3 => Some(OpCode::OpNegate),
        _ => None,
    }
}
