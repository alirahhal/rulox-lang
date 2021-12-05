use std::error;

use derivative::*; // 2.2.0

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
    OpPrint,
    OpPop,

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
        8 => Some(OpCode::OpNil),
        9 => Some(OpCode::OpTrue),
        10 => Some(OpCode::OpFalse),
        11 => Some(OpCode::OpNot),
        12 => Some(OpCode::OpEqual),
        13 => Some(OpCode::OpGreater),
        14 => Some(OpCode::OpLess),
        15 => Some(OpCode::OpPrint),
        16 => Some(OpCode::OpPop),

        _ => None,
    }
}

#[repr(u8)]
#[derive(Debug, Eq, Derivative, Copy, Clone)]
#[derivative(PartialEq, Hash)]
pub enum TokenType {
    // Single-character tokens.
    TokenLeftParen,
    TokenRightParen,
    TokenLeftBrace,
    TokenRightBrace,
    TokenComma,
    TokenDot,
    TokenMinus,
    TokenPlus,
    TokenSemicolon,
    TokenSlash,
    TokenStar,

    // One or two character tokens.
    TokenBang,
    TokenBangEqual,
    TokenEqual,
    TokenEqualEqual,
    TokenGreater,
    TokenGreaterEqual,
    TokenLess,
    TokenLessEqual,

    // Literals.
    TokenIdentifier,
    TokenString,
    TokenNumber,

    // Keywords.
    TokenAnd,
    TokenClass,
    TokenElse,
    TokenFalse,
    TokenFor,
    TokenFun,
    TokenIf,
    TokenNil,
    TokenOr,
    TokenPrint,
    TokenReturn,
    TokenSuper,
    TokenThis,
    TokenTrue,
    TokenVar,
    TokenWhile,

    TokenError,
    TokenEof,

    Unknown,
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Unknown
    }
}

// pub fn token_type_from_u8(n: u8) -> Option<TokenType> {
//     match n {
//         0 => Some(OpCode::OpReturn),
//         1 => Some(OpCode::OpConstant),
//         2 => Some(OpCode::OpConstantLong),
//         3 => Some(OpCode::OpAdd),
//         4 => Some(OpCode::OpSubstract),
//         5 => Some(OpCode::OpMultiply),
//         6 => Some(OpCode::OpDivide),
//         7 => Some(OpCode::OpNegate),
//         _ => None,
//     }
// }

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Precedence {
    PrecNone,
    PrecAssignment, // =
    PrecOr,         // or
    PrecAnd,        // and
    PrecEquality,   // == !=
    PrecComparison, // < > <= >=
    PrecTerm,       // + -
    PrecFactor,     // * /
    PrecUnary,      // ! -
    PrecCall,       // . ()
    PrecPrimary,

    Unknown,
}

impl Default for Precedence {
    fn default() -> Self {
        Precedence::Unknown
    }
}

pub fn precedence_from_u8(n: u8) -> Option<Precedence> {
    match n {
        0 => Some(Precedence::PrecNone),
        1 => Some(Precedence::PrecAssignment),
        2 => Some(Precedence::PrecOr),
        3 => Some(Precedence::PrecAnd),
        4 => Some(Precedence::PrecEquality),
        5 => Some(Precedence::PrecComparison),
        6 => Some(Precedence::PrecTerm),
        7 => Some(Precedence::PrecFactor),
        8 => Some(Precedence::PrecUnary),
        9 => Some(Precedence::PrecCall),
        10 => Some(Precedence::PrecPrimary),
        _ => None,
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ValueType {
    ValBool,
    ValNil,
    ValNumber,
}
