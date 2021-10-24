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
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::TokenEof);
        }

        self.error_token("Unexpected character.".to_string())
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
