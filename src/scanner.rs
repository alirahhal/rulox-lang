use crate::{common::TokenType, main};

// TODO: Change [start] and [current] type to pointer for performance enhancement
pub struct Scanner<'a> {
    pub source: &'a String,
    pub start: i32,
    pub current: i32,
    pub line: i32,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32,
}

impl<'a> Scanner<'a> {
    pub fn init_scanner(source: &'a String) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::TokenEof);
        }

        let c = self.advance();

        if c.is_digit(10) {
            return self.number();
        }

        match c {
            '(' => return self.make_token(TokenType::TokenLeftParen),
            ')' => return self.make_token(TokenType::TokenRightParen),
            '{' => return self.make_token(TokenType::TokenLeftBrace),
            '}' => return self.make_token(TokenType::TokenRightBrace),
            ';' => return self.make_token(TokenType::TokenSemicolon),
            ',' => return self.make_token(TokenType::TokenComma),
            '.' => return self.make_token(TokenType::TokenDot),
            '-' => return self.make_token(TokenType::TokenMinus),
            '+' => return self.make_token(TokenType::TokenPlus),
            '/' => return self.make_token(TokenType::TokenSlash),
            '*' => return self.make_token(TokenType::TokenStar),
            '!' => {
                let token_type = if self.match_token('=') {
                    TokenType::TokenBangEqual
                } else {
                    TokenType::TokenBang
                };
                return self.make_token(token_type);
            }
            '=' => {
                let token_type = if self.match_token('=') {
                    TokenType::TokenEqualEqual
                } else {
                    TokenType::TokenEqual
                };
                return self.make_token(token_type);
            }
            '<' => {
                let token_type = if self.match_token('=') {
                    TokenType::TokenLessEqual
                } else {
                    TokenType::TokenLess
                };
                return self.make_token(token_type);
            }
            '>' => {
                let token_type = if self.match_token('=') {
                    TokenType::TokenGreaterEqual
                } else {
                    TokenType::TokenGreater
                };
                return self.make_token(token_type);
            }
            '"' => return self.string(),
            _ => return self.error_token("Unexpected character.".to_string()),
        }
    }

    fn advance(&mut self) -> char {
        self.current = self.current + 1;
        self.source
            .chars()
            .nth((self.current - 1) as usize)
            .unwrap()
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line = self.line + 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        // A comment goes until the end of the line.
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn peek(&mut self) -> char {
        return self.source.chars().nth((self.current) as usize).unwrap();
    }

    fn peek_next(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self
            .source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap();
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.".to_string());
        }

        // The closing quote.
        self.advance();
        return self.make_token(TokenType::TokenString);
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth((self.current) as usize).unwrap() != expected {
            return false;
        }

        self.current = self.current + 1;
        true
    }

    fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume the ".".
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.make_token(TokenType::TokenNumber)
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.chars().count() as i32
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let token = Token {
            token_type,
            lexeme: self.source[(self.start as usize)..(self.start as usize)].to_string(),
            line: self.line,
        };
        token
    }

    fn error_token(&mut self, message: String) -> Token {
        let token = Token {
            token_type: TokenType::TokenError,
            lexeme: message,
            line: self.line,
        };
        token
    }
}
