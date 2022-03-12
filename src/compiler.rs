use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;

use crate::{
    chunk::Chunk,
    common::{precedence_from_u8, OpCode, Precedence},
    debug::disassemble_chunk,
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

pub fn compile(source: &str) -> Result<Chunk, ()> {
    let mut scanner = Scanner::new(source);
    let mut chunk = Chunk::new();
    let mut parser = Parser::new(&mut scanner, &mut chunk);

    parser.parse();

    Ok(chunk)
    // !parser.had_error
}

pub type ParseFn = fn(&mut Parser, can_assign: bool) -> ();

pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}

pub struct Local {
    pub name: Token,
    pub depth: i32,
}

pub struct Compiler {
    pub locals: Vec<Local>,
    pub scope_depth: i32,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            scope_depth: 0,
            locals: Vec::new(),
        }
    }

    pub fn add_local(&mut self, name: &Token) {
        let local = Local {
            name: name.clone(),
            depth: -1,
        };
        self.locals.push(local);
    }

    pub fn local_at(&self, index: usize) -> &Local {
        &self.locals[index] as _
    }

    pub fn update_local_depth_at(&mut self, index: usize, depth: i32) {
        self.locals[index].depth = depth;
    }
}

lazy_static! {
    static ref PARSER_RULES: HashMap<TokenType, ParseRule> = hashmap! {
        TokenType::TokenLeftParen => ParseRule {
            prefix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::grouping(parser, can_assign)
            }),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenRightParen => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenLeftBrace => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenRightBrace => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenMinus => ParseRule {
            prefix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::unary(parser, can_assign)
            }),
            infix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::binary(parser, can_assign)
            }),
            precedence: Precedence::Term,
        },
        TokenType::TokenPlus => ParseRule {
            prefix: None,
            infix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::binary(parser, can_assign)
            }),
            precedence: Precedence::Term,
        },
        TokenType::TokenSemicolon => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenSlash => ParseRule {
            prefix: None,
            infix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::binary(parser, can_assign)
            }),
            precedence: Precedence::Factor,
        },
        TokenType::TokenStar => ParseRule {
            prefix: None,
            infix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::binary(parser, can_assign)
            }),
            precedence: Precedence::Factor,
        },
        TokenType::TokenBang => ParseRule {
            prefix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::unary(parser, can_assign)
            }),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenBangEqual => ParseRule {
            prefix: None,
            infix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::binary(parser, can_assign)
            }),
            precedence: Precedence::Equality,
        },
        TokenType::TokenEqualEqual => ParseRule {
            prefix: None,
            infix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::binary(parser, can_assign)
            }),
            precedence: Precedence::Equality,
        },
        TokenType::TokenGreater => ParseRule {
            prefix: None,
            infix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::binary(parser, can_assign)
            }),
            precedence: Precedence::Comparison,
        },
        TokenType::TokenGreaterEqual => ParseRule {
            prefix: None,
            infix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::binary(parser, can_assign)
            }),
            precedence: Precedence::Comparison,
        },
        TokenType::TokenLess => ParseRule {
            prefix: None,
            infix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::binary(parser, can_assign)
            }),
            precedence: Precedence::Comparison,
        },
        TokenType::TokenLessEqual => ParseRule {
            prefix: None,
            infix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::binary(parser, can_assign)
            }),
            precedence: Precedence::Comparison,
        },
        TokenType::TokenIdentifier => ParseRule {
            prefix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::variable(parser, can_assign)
            }),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenString => ParseRule {
            prefix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::string(parser, can_assign)
            }),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenNumber => ParseRule {
            prefix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::number(parser, can_assign)
            }),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenFalse => ParseRule {
            prefix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::literal(parser, can_assign)
            }),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenTrue => ParseRule {
            prefix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::literal(parser, can_assign)
            }),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenNil => ParseRule {
            prefix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::literal(parser, can_assign)
            }),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenPrint => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenIf => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenElse => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenVar => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::TokenAnd => ParseRule {
            prefix: None,
            infix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::and_(parser, can_assign)
            }),
            precedence: Precedence::And,
        },
        TokenType::TokenOr => ParseRule {
            prefix: None,
            infix: Some(|parser: &mut Parser<'_>, can_assign: bool| {
                Parser::or_(parser, can_assign)
            }),
            precedence: Precedence::Or,
        },
        TokenType::TokenEof => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    };
}

pub struct Parser<'a> {
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,

    pub scanner: &'a mut Scanner<'a>,
    pub chunk: &'a mut Chunk,
    pub current_compiler: Compiler,
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

            scanner,
            chunk,
            current_compiler: Compiler::new(),
        }
    }

    pub fn parse(&mut self) {
        self.advance();

        while !self.match_token_type(TokenType::TokenEof) {
            self.declaration();
        }

        self.end_compiler();
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

    fn match_token_type(&mut self, token_type: TokenType) -> bool {
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

    fn emit_loop(&mut self, loop_start: i32) {
        self.emit_byte(OpCode::OpLoop as u8);

        let offset = self.current_chunk().code.len() as i32 - loop_start + 2;

        self.emit_byte(((offset >> 8) & 0xff) as u8);
        self.emit_byte((offset & 0xff) as u8);
    }

    fn emit_jump(&mut self, instruction: u8) -> i32 {
        self.emit_byte(instruction);
        self.emit_byte(0xff);
        self.emit_byte(0xff);
        (self.current_chunk().code.len() - 2) as i32
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn as u8)
    }

    fn emit_constant(&mut self, value: Value) {
        let line = self.previous.line;
        self.current_chunk().write_constant(value, line);
    }

    fn patch_jump(&mut self, offset: i32) {
        // -2 to adjust for the bytecode for the jump offset itself.
        let jump = self.current_chunk().code.len() as i32 - offset - 2;

        // if (jump > UINT16_MAX) {
        // 	error("Too much code to jump over.");
        //   }

        self.current_chunk().code[offset as usize] = ((jump >> 8) & 0xff) as u8;
        self.current_chunk().code[(offset + 1) as usize] = (jump & 0xff) as u8;
    }

    fn end_compiler(&mut self) {
        self.emit_return();
        if !self.had_error {
            disassemble_chunk(self.current_chunk(), "code".to_string());
        }
    }

    fn begin_scope(&mut self) {
        self.current_compiler.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.current_compiler.scope_depth -= 1;

        while !self.current_compiler.locals.is_empty()
            && self
                .current_compiler
                .local_at(self.current_compiler.locals.len() - 1)
                .depth
                > self.current_compiler.scope_depth
        {
            self.emit_byte(OpCode::OpPop as u8);
            self.current_compiler.locals.pop();
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunk
    }

    fn binary(&mut self, _can_assign: bool) {
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
            TokenType::TokenMinus => self.emit_byte(OpCode::OpSubtract as u8),
            TokenType::TokenStar => self.emit_byte(OpCode::OpMultiply as u8),
            TokenType::TokenSlash => self.emit_byte(OpCode::OpDivide as u8),
            _ => (),
        }
    }

    fn literal(&mut self, _can_assign: bool) {
        match self.previous.token_type {
            TokenType::TokenFalse => self.emit_byte(OpCode::OpFalse as u8),
            TokenType::TokenTrue => self.emit_byte(OpCode::OpTrue as u8),
            TokenType::TokenNil => self.emit_byte(OpCode::OpNil as u8),
            _ => (),
        }
    }

    fn grouping(&mut self, _can_assign: bool) {
        self.expression();
        self.consume(
            TokenType::TokenRightParen,
            "Expect ')' after expression.".to_string(),
        );
    }

    fn number(&mut self, _can_assign: bool) {
        let value: Value = Value::new_number(self.previous.lexeme.parse().unwrap());
        self.emit_constant(value);
    }

    fn or_(&mut self, _can_assign: bool) {
        let else_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8);
        let end_jump = self.emit_jump(OpCode::OpJump as u8);

        self.patch_jump(else_jump);
        self.emit_byte(OpCode::OpPop as u8);

        self.parse_precedence(Precedence::Or);
        self.patch_jump(end_jump);
    }

    fn string(&mut self, _can_assign: bool) {
        let slen = self.previous.lexeme.len();
        let value = Value::new_obj_string(self.previous.lexeme[1..slen - 1].to_string());
        self.emit_constant(value);
    }

    fn variable(&mut self, can_assign: bool) {
        let prev = self.previous.to_owned();
        self.named_variable(&prev, can_assign);
    }

    fn named_variable(&mut self, name: &Token, can_assign: bool) {
        let get_op: OpCode;
        let set_op: OpCode;
        let get_op_long: OpCode;
        let set_op_long: OpCode;
        let mut arg = self.resolve_local(name);

        if arg != -1 {
            get_op = OpCode::OpGetLocal;
            set_op = OpCode::OpSetLocal;
            get_op_long = OpCode::OpGetLocalLong;
            set_op_long = OpCode::OpSetLocalLong;
        } else {
            arg = self.identifier_constant(name);
            get_op = OpCode::OpGetGlobal;
            set_op = OpCode::OpSetGlobal;
            get_op_long = OpCode::OpGetGlobalLong;
            set_op_long = OpCode::OpSetGlobalLong;
        }

        if can_assign && self.match_token_type(TokenType::TokenEqual) {
            self.expression();
            if arg < 256 {
                self.emit_bytes(set_op as u8, arg as u8);
            } else {
                self.emit_byte(set_op_long as u8);
                self.emit_byte((arg & 0xff) as u8);
                self.emit_byte(((arg >> 8) & 0xff) as u8);
                self.emit_byte(((arg >> 16) & 0xff) as u8);
            }
        } else if arg < 256 {
            self.emit_bytes(get_op as u8, arg as u8);
        } else {
            self.emit_byte(get_op_long as u8);
            self.emit_byte((arg & 0xff) as u8);
            self.emit_byte(((arg >> 8) & 0xff) as u8);
            self.emit_byte(((arg >> 16) & 0xff) as u8);
        }
    }

    fn unary(&mut self, _can_assign: bool) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::TokenBang => self.emit_byte(OpCode::OpNot as u8),
            TokenType::TokenMinus => self.emit_byte(OpCode::OpNegate as u8),
            _ => (),
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

        let can_assign = precedence as u8 <= Precedence::Assignment as u8;
        prefix_rule_fn(self, can_assign);

        while precedence as u8 <= self.get_rule(self.current.token_type).precedence as u8 {
            self.advance();
            let infix_rule_fn = self.get_rule(self.previous.token_type).infix.unwrap();
            infix_rule_fn(self, can_assign);
        }

        if can_assign && self.match_token_type(TokenType::TokenEqual) {
            self.error("Invalid assignmet target.".to_string());
        }
    }

    fn define_variable(&mut self, global: i32) {
        if self.current_compiler.scope_depth > 0 {
            self.mark_initialized();
            return;
        }
        if global < 256 {
            self.emit_bytes(OpCode::OpDefineGlobal as u8, global as u8);
        } else {
            self.emit_byte(OpCode::OpDefineGlobalLong as u8);
            self.emit_byte((global & 0xff) as u8);
            self.emit_byte(((global >> 8) & 0xff) as u8);
            self.emit_byte(((global >> 16) & 0xff) as u8);
        }
    }

    fn and_(&mut self, _can_assign: bool) {
        let end_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8);

        self.emit_byte(OpCode::OpPop as u8);
        self.parse_precedence(Precedence::And);

        self.patch_jump(end_jump);
    }

    fn parse_variable(&mut self, error_message: String) -> i32 {
        self.consume(TokenType::TokenIdentifier, error_message);

        self.declare_variable();
        if self.current_compiler.scope_depth > 0 {
            return 0;
        }

        let prev = &self.previous.clone();
        self.identifier_constant(prev)
    }

    fn mark_initialized(&mut self) {
        self.current_compiler.update_local_depth_at(
            self.current_compiler.locals.len() - 1,
            self.current_compiler.scope_depth,
        );
    }

    fn identifier_constant(&mut self, name: &Token) -> i32 {
        self.current_chunk()
            .add_constant(Value::new_obj_string(name.lexeme.to_owned()))
    }

    fn resolve_local(&mut self, name: &Token) -> i32 {
        let mut i = self.current_compiler.locals.len() as i32 - 1;
        while i >= 0 {
            let local = self.current_compiler.local_at(i as usize);
            if name.lexeme == local.name.lexeme {
                if local.depth == -1 {
                    self.error("Can't read local variable in its own initializer.".to_string());
                }
                return i;
            }

            i -= 1;
        }

        -1
    }

    fn declare_variable(&mut self) {
        if self.current_compiler.scope_depth == 0 {
            return;
        }

        let mut error_flagged: bool = false;
        let name = &self.previous;
        let mut i = self.current_compiler.locals.len() as i32 - 1;
        while i >= 0 {
            let local = self.current_compiler.local_at(i as usize);
            if local.depth != -1 && local.depth < self.current_compiler.scope_depth {
                break;
            }

            i -= 1;
            if name.lexeme == local.name.lexeme {
                error_flagged = true;
                break;
            }
        }
        if !error_flagged {
            self.current_compiler.add_local(name);
        } else {
            self.error("Already a variable with this name in this scope.".to_string());
        }
    }

    fn get_rule(&self, token_type: TokenType) -> &ParseRule {
        &PARSER_RULES[&token_type]
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn block(&mut self) {
        while !self.check(TokenType::TokenRightBrace) && !self.check(TokenType::TokenEof) {
            self.declaration();
        }

        self.consume(
            TokenType::TokenRightBrace,
            "Expect '}' after block.".to_string(),
        );
    }

    fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.".to_string());

        if self.match_token_type(TokenType::TokenEqual) {
            self.expression();
        } else {
            self.emit_byte(OpCode::OpNil as u8)
        }
        self.consume(
            TokenType::TokenSemicolon,
            "Expect ';' after variable declaration.".to_string(),
        );

        self.define_variable(global);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(
            TokenType::TokenSemicolon,
            "Expect ';' after expression.".to_string(),
        );
        self.emit_byte(OpCode::OpPop as u8);
    }

    fn for_statement(&mut self) {
        self.begin_scope();
        self.consume(
            TokenType::TokenLeftParen,
            "Expect '(' after 'for'.".to_string(),
        );
        if self.match_token_type(TokenType::TokenSemicolon) {
            // No initializer
        } else if self.match_token_type(TokenType::TokenVar) {
            self.var_declaration();
        } else {
            self.expression_statement();
        }

        let mut loop_start = self.current_chunk().code.len() as i32;
        let mut exit_jump = -1;
        if !self.match_token_type(TokenType::TokenSemicolon) {
            self.expression();
            self.consume(
                TokenType::TokenSemicolon,
                "Expect ';' after loop condition.".to_string(),
            );

            // Jump out of the loop if the condition is false.
            exit_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8);
            self.emit_byte(OpCode::OpPop as u8); // Condition.
        }

        if !self.match_token_type(TokenType::TokenRightParen) {
            let body_jump = self.emit_jump(OpCode::OpJump as u8);
            let increment_start = self.current_chunk().code.len() as i32;
            self.expression();
            self.emit_byte(OpCode::OpPop as u8);
            self.consume(
                TokenType::TokenRightParen,
                "Expect ')' after for clauses.".to_string(),
            );

            self.emit_loop(loop_start);
            loop_start = increment_start;
            self.patch_jump(body_jump);
        }

        self.statement();
        self.emit_loop(loop_start);

        if exit_jump != -1 {
            self.patch_jump(exit_jump);
            self.emit_byte(OpCode::OpPop as u8);
        }
        self.end_scope();
    }

    fn if_statement(&mut self) {
        self.consume(
            TokenType::TokenLeftParen,
            "Expect '(' after 'if'.".to_string(),
        );
        self.expression();
        self.consume(
            TokenType::TokenRightParen,
            "Expect ')' after condition.".to_string(),
        );

        let then_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8);
        self.emit_byte(OpCode::OpPop as u8);
        self.statement();

        let else_jump = self.emit_jump(OpCode::OpJump as u8);

        self.patch_jump(then_jump);
        self.emit_byte(OpCode::OpPop as u8);

        if self.match_token_type(TokenType::TokenElse) {
            self.statement();
        }
        self.patch_jump(else_jump);
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(
            TokenType::TokenSemicolon,
            "Expect ';' after value.".to_string(),
        );
        self.emit_byte(OpCode::OpPrint as u8);
    }

    fn while_statement(&mut self) {
        let loop_start = self.current_chunk().code.len() as i32;
        self.consume(
            TokenType::TokenLeftParen,
            "Expect '(' after 'while'.".to_string(),
        );
        self.expression();
        self.consume(
            TokenType::TokenRightParen,
            "Expect ')' after 'while'.".to_string(),
        );

        let exit_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8);
        self.emit_byte(OpCode::OpPop as u8);
        self.statement();
        self.emit_loop(loop_start);

        self.patch_jump(exit_jump);
        self.emit_byte(OpCode::OpPop as u8);
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

            self.advance();
        }
    }

    fn declaration(&mut self) {
        if self.match_token_type(TokenType::TokenVar) {
            self.var_declaration();
        } else {
            self.statement();
        }

        if self.panic_mode {
            self.synchronize();
        }
    }

    fn statement(&mut self) {
        if self.match_token_type(TokenType::TokenPrint) {
            self.print_statement();
        } else if self.match_token_type(TokenType::TokenLeftBrace) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else if self.match_token_type(TokenType::TokenIf) {
            self.if_statement();
        } else if self.match_token_type(TokenType::TokenWhile) {
            self.while_statement();
        } else if self.match_token_type(TokenType::TokenFor) {
            self.for_statement();
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
