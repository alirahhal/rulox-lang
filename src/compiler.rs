use crate::{chunk::Chunk, common::{OpCode, TokenType}, scanner::{Scanner, Token}};

pub fn compile(source: &String, chunk: &mut Chunk) -> bool {
    let mut scanner = Scanner::init_scanner(source);

    let mut parser = Parser::new(&mut scanner, chunk);

    parser.advance();
    //   expression();
    parser.consume(TokenType::TokenEof, "Expect end of expression.".to_string());
    parser.end_compiler();
    !parser.had_error
}

pub struct Parser<'a> {
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,

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
            scanner,
            chunk,
        }
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            self.current = self.scanner.scan_token();
            println!("current token: {:?}", self.current.clone().lexeme);
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

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunk
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
