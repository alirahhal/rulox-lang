use std::{array::IntoIter, collections::HashMap, iter::FromIterator};

use crate::{
    chunk::Chunk,
    common::{precedence_from_u8, OpCode, Precedence, TokenType},
    debug::disassemble_chunk,
    object::{Obj, ObjString, ObjType},
    scanner::{Scanner, Token},
    value::Value,
};

pub fn compile(source: &String, chunk: &mut Chunk) -> bool {
    let mut scanner = Scanner::init_scanner(source);

    let mut parser = Parser::new(&mut scanner, chunk);

    parser.advance();

    while !parser.match_token_type(TokenType::TokenEof) {
        parser.declaration();
    }

    parser.end_compiler();
    !parser.had_error
}

pub type ParseFn = fn(&mut Parser) -> ();

pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}

pub struct Parser<'a> {
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,
    pub parse_rule: HashMap<TokenType, ParseRule>,

    pub scanner: &'a mut Scanner<'a>,
    pub chunk: &'a mut Chunk,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: &'a mut Scanner<'a>, chunk: &'a mut Chunk) -> Self {
        Parser {
            current: Token {
                token_type: TokenType::Unknown,
                lexeme: "".to_string(),
                line: -1,
            },
            previous: Token {
                token_type: TokenType::Unknown,
                lexeme: "".to_string(),
                line: -1,
            },
            had_error: false,
            panic_mode: false,
            parse_rule: HashMap::<TokenType, ParseRule>::from_iter(IntoIter::new([
                (
                    TokenType::TokenLeftParen,
                    ParseRule {
                        prefix: Some(|parser: &mut Parser<'_>| Parser::grouping(parser)),
                        infix: None,
                        precedence: Precedence::PrecNone,
                    },
                ),
                (
                    TokenType::TokenRightParen,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::PrecNone,
                    },
                ),
                (
                    TokenType::TokenMinus,
                    ParseRule {
                        prefix: Some(|parser: &mut Parser<'_>| Parser::unary(parser)),
                        infix: Some(|parser: &mut Parser<'_>| Parser::binary(parser)),
                        precedence: Precedence::PrecTerm,
                    },
                ),
                (
                    TokenType::TokenPlus,
                    ParseRule {
                        prefix: None,
                        infix: Some(|parser: &mut Parser<'_>| Parser::binary(parser)),
                        precedence: Precedence::PrecTerm,
                    },
                ),
                (
                    TokenType::TokenSemicolon,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::PrecNone,
                    },
                ),
                (
                    TokenType::TokenSlash,
                    ParseRule {
                        prefix: None,
                        infix: Some(|parser: &mut Parser<'_>| Parser::binary(parser)),
                        precedence: Precedence::PrecFactor,
                    },
                ),
                (
                    TokenType::TokenStar,
                    ParseRule {
                        prefix: None,
                        infix: Some(|parser: &mut Parser<'_>| Parser::binary(parser)),
                        precedence: Precedence::PrecFactor,
                    },
                ),
                (
                    TokenType::TokenBang,
                    ParseRule {
                        prefix: Some(|parser: &mut Parser<'_>| Parser::unary(parser)),
                        infix: None,
                        precedence: Precedence::PrecNone,
                    },
                ),
                (
                    TokenType::TokenBangEqual,
                    ParseRule {
                        prefix: None,
                        infix: Some(|parser: &mut Parser<'_>| Parser::binary(parser)),
                        precedence: Precedence::PrecEquality,
                    },
                ),
                (
                    TokenType::TokenEqualEqual,
                    ParseRule {
                        prefix: None,
                        infix: Some(|parser: &mut Parser<'_>| Parser::binary(parser)),
                        precedence: Precedence::PrecEquality,
                    },
                ),
                (
                    TokenType::TokenGreater,
                    ParseRule {
                        prefix: None,
                        infix: Some(|parser: &mut Parser<'_>| Parser::binary(parser)),
                        precedence: Precedence::PrecComparison,
                    },
                ),
                (
                    TokenType::TokenGreaterEqual,
                    ParseRule {
                        prefix: None,
                        infix: Some(|parser: &mut Parser<'_>| Parser::binary(parser)),
                        precedence: Precedence::PrecComparison,
                    },
                ),
                (
                    TokenType::TokenLess,
                    ParseRule {
                        prefix: None,
                        infix: Some(|parser: &mut Parser<'_>| Parser::binary(parser)),
                        precedence: Precedence::PrecComparison,
                    },
                ),
                (
                    TokenType::TokenLessEqual,
                    ParseRule {
                        prefix: None,
                        infix: Some(|parser: &mut Parser<'_>| Parser::binary(parser)),
                        precedence: Precedence::PrecComparison,
                    },
                ),
                (
                    TokenType::TokenString,
                    ParseRule {
                        prefix: Some(|parser: &mut Parser<'_>| Parser::string(parser)),
                        infix: None,
                        precedence: Precedence::PrecNone,
                    },
                ),
                (
                    TokenType::TokenNumber,
                    ParseRule {
                        prefix: Some(|parser: &mut Parser<'_>| Parser::number(parser)),
                        infix: None,
                        precedence: Precedence::PrecNone,
                    },
                ),
                (
                    TokenType::TokenFalse,
                    ParseRule {
                        prefix: Some(|parser: &mut Parser<'_>| Parser::literal(parser)),
                        infix: None,
                        precedence: Precedence::PrecNone,
                    },
                ),
                (
                    TokenType::TokenTrue,
                    ParseRule {
                        prefix: Some(|parser: &mut Parser<'_>| Parser::literal(parser)),
                        infix: None,
                        precedence: Precedence::PrecNone,
                    },
                ),
                (
                    TokenType::TokenNil,
                    ParseRule {
                        prefix: Some(|parser: &mut Parser<'_>| Parser::literal(parser)),
                        infix: None,
                        precedence: Precedence::PrecNone,
                    },
                ),
                (
                    TokenType::TokenPrint,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::PrecNone,
                    },
                ),
                (
                    TokenType::TokenEof,
                    ParseRule {
                        prefix: None,
                        infix: None,
                        precedence: Precedence::PrecNone,
                    },
                ),
            ])),

            scanner,
            chunk,
        }
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            self.current = self.scanner.scan_token();
            if self.current.token_type != TokenType::TokenError {
                break;
            }

            self.error_at_current(self.current.lexeme.to_string());
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        self.current.token_type == token_type
    }

    pub fn match_token_type(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false;
        }
        self.advance();
        true
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.previous.line;
        self.current_chunk().write_chunk(byte, line);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn as u8)
    }

    fn emit_constant(&mut self, value: Value) {
        let line = self.previous.line;
        self.current_chunk().write_constant(value, line);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
        if !self.had_error {
            disassemble_chunk(self.current_chunk(), "code".to_string());
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunk
    }

    fn binary(&mut self) {
        let operator_type = self.previous.token_type;
        let rule = self.get_rule(operator_type);
        let precedence = precedence_from_u8(rule.precedence as u8 + 1).unwrap();
        self.parse_precedence(precedence);

        match operator_type {
            TokenType::TokenBangEqual => {
                self.emit_bytes(OpCode::OpEqual as u8, OpCode::OpNot as u8)
            }
            TokenType::TokenEqualEqual => self.emit_byte(OpCode::OpEqual as u8),
            TokenType::TokenGreater => self.emit_byte(OpCode::OpGreater as u8),
            TokenType::TokenGreaterEqual => {
                self.emit_bytes(OpCode::OpLess as u8, OpCode::OpNot as u8)
            }
            TokenType::TokenLess => self.emit_byte(OpCode::OpLess as u8),
            TokenType::TokenLessEqual => {
                self.emit_bytes(OpCode::OpGreater as u8, OpCode::OpNot as u8)
            }
            TokenType::TokenPlus => self.emit_byte(OpCode::OpAdd as u8),
            TokenType::TokenMinus => self.emit_byte(OpCode::OpSubstract as u8),
            TokenType::TokenStar => self.emit_byte(OpCode::OpMultiply as u8),
            TokenType::TokenSlash => self.emit_byte(OpCode::OpDivide as u8),
            _ => return,
        }
    }

    fn literal(&mut self) {
        match self.previous.token_type {
            TokenType::TokenFalse => self.emit_byte(OpCode::OpFalse as u8),
            TokenType::TokenTrue => self.emit_byte(OpCode::OpTrue as u8),
            TokenType::TokenNil => self.emit_byte(OpCode::OpNil as u8),
            _ => return,
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(
            TokenType::TokenRightParen,
            "Expect ')' after expression.".to_string(),
        );
    }

    fn number(&mut self) {
        let value: Value = Value::new_number(self.previous.lexeme.parse().unwrap());
        self.emit_constant(value);
    }

    fn string(&mut self) {
        // let value: Value = Value::new_number(self.previous.lexeme.parse().unwrap());
        let slen = self.previous.lexeme.len();
        let obj_s = ObjString {
            obj: Obj {
                obj_type: ObjType::ObjString,
            },
            string: self.previous.lexeme[1..slen - 1].to_string(),
        };

        self.emit_constant(value);
    }

    fn unary(&mut self) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(Precedence::PrecUnary);

        match operator_type {
            TokenType::TokenBang => self.emit_byte(OpCode::OpNot as u8),
            TokenType::TokenMinus => self.emit_byte(OpCode::OpNegate as u8),
            _ => return,
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.get_rule(self.previous.token_type).prefix;
        let prefix_rule_fn = match prefix_rule {
            Some(rule_fn) => rule_fn,
            None => {
                self.error("Expect expression.".to_string());
                return;
            }
        };

        prefix_rule_fn(self);

        while precedence as u8 <= self.get_rule(self.current.token_type).precedence as u8 {
            self.advance();
            let infix_rule_fn = self.get_rule(self.previous.token_type).infix.unwrap();
            infix_rule_fn(self);
        }
    }

    fn get_rule(&self, token_type: TokenType) -> &ParseRule {
        &self.parse_rule[&token_type]
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::PrecAssignment);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(
            TokenType::TokenSemicolon,
            "Expect ';' after expression.".to_string(),
        );
        self.emit_byte(OpCode::OpPop as u8);
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(
            TokenType::TokenSemicolon,
            "Expect ';' after value.".to_string(),
        );
        self.emit_byte(OpCode::OpPrint as u8);
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;

        while self.current.token_type != TokenType::TokenEof {
            if self.previous.token_type == TokenType::TokenSemicolon {
                return;
            }

            match self.current.token_type {
                TokenType::TokenClass
                | TokenType::TokenFun
                | TokenType::TokenVar
                | TokenType::TokenFor
                | TokenType::TokenIf
                | TokenType::TokenWhile
                | TokenType::TokenPrint
                | TokenType::TokenReturn => {
                    return;
                }
                _ => (),
            }
        }

        self.advance();
    }

    fn declaration(&mut self) {
        self.statement();

        if self.panic_mode {
            self.synchronize();
        }
    }

    fn statement(&mut self) {
        if self.match_token_type(TokenType::TokenPrint) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    fn error_at(&mut self, token: Token, message: String) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        print!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::TokenEof => print!(" at end"),
            TokenType::TokenError => (),
            _ => print!(" at '{}'", token.lexeme),
        }

        println!(": {}", message);
        self.had_error = true;
    }

    fn error(&mut self, message: String) {
        self.error_at(self.previous.clone(), message);
    }

    fn error_at_current(&mut self, message: String) {
        self.error_at(self.current.clone(), message);
    }
}
