use crate::common::TokenType;

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

        if c.is_alphabetic() {
            return self.identifier();
        }
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

    fn identifier_type(&mut self) -> TokenType {
        match self.source.chars().nth((self.start) as usize).unwrap() {
            'a' => return self.check_keyword(1, 2, "nd".to_string(), TokenType::TokenAnd),
            'c' => return self.check_keyword(1, 4, "lass".to_string(), TokenType::TokenClass),
            'e' => return self.check_keyword(1, 3, "lse".to_string(), TokenType::TokenElse),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth((self.start + 1) as usize).unwrap() {
                        'a' => {
                            return self.check_keyword(
                                2,
                                3,
                                "lse".to_string(),
                                TokenType::TokenFalse,
                            )
                        }
                        'o' => {
                            return self.check_keyword(2, 1, "r".to_string(), TokenType::TokenFor)
                        }
                        'u' => {
                            return self.check_keyword(2, 1, "n".to_string(), TokenType::TokenFun)
                        }
                        _ => (),
                    }
                }
            }
            'i' => return self.check_keyword(1, 1, "f".to_string(), TokenType::TokenIf),
            'n' => return self.check_keyword(1, 2, "il".to_string(), TokenType::TokenNil),
            'o' => return self.check_keyword(1, 1, "r".to_string(), TokenType::TokenOr),
            'p' => return self.check_keyword(1, 4, "rint".to_string(), TokenType::TokenPrint),
            'r' => return self.check_keyword(1, 5, "eturn".to_string(), TokenType::TokenReturn),
            's' => return self.check_keyword(1, 4, "uper".to_string(), TokenType::TokenSuper),
            't' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth((self.start + 1) as usize).unwrap() {
                        'h' => {
                            return self.check_keyword(2, 2, "is".to_string(), TokenType::TokenThis)
                        }
                        'r' => {
                            return self.check_keyword(2, 2, "ue".to_string(), TokenType::TokenTrue)
                        }
                        _ => (),
                    }
                }
            }
            'v' => return self.check_keyword(1, 2, "ar".to_string(), TokenType::TokenVar),
            'w' => return self.check_keyword(1, 4, "hile".to_string(), TokenType::TokenWhile),
            _ => (),
        }

        TokenType::TokenIdentifier
    }

    fn check_keyword(
        &mut self,
        start: i32,
        length: i32,
        rest: String,
        token_type: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + length
            && self.source[(self.start + start) as usize..(self.start + start + length) as usize]
                == rest
        {
            return token_type;
        }

        TokenType::TokenIdentifier
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphabetic() || self.peek().is_digit(10) {
            self.advance();
        }

        let id = self.identifier_type();
        self.make_token(id)
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