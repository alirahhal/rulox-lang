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

#[derive(Debug, Eq, Copy, Clone, PartialEq, Hash)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,

    Unknown,
}

impl Default for Precedence {
    fn default() -> Self {
        Precedence::Unknown
    }
}

pub fn precedence_from_u8(n: u8) -> Option<Precedence> {
    match n {
        0 => Some(Precedence::None),
        1 => Some(Precedence::Assignment),
        2 => Some(Precedence::Or),
        3 => Some(Precedence::And),
        4 => Some(Precedence::Equality),
        5 => Some(Precedence::Comparison),
        6 => Some(Precedence::Term),
        7 => Some(Precedence::Factor),
        8 => Some(Precedence::Unary),
        9 => Some(Precedence::Call),
        10 => Some(Precedence::Primary),
        _ => None,
    }
}
