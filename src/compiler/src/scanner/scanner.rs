use super::token::{Token, TokenType};

pub struct Scanner<'a> {
    pub source: &'a str,
    pub start: i32,
    pub current: i32,
    pub line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
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
            '(' => self.make_token(TokenType::TokenLeftParen),
            ')' => self.make_token(TokenType::TokenRightParen),
            '{' => self.make_token(TokenType::TokenLeftBrace),
            '}' => self.make_token(TokenType::TokenRightBrace),
            ';' => self.make_token(TokenType::TokenSemicolon),
            ',' => self.make_token(TokenType::TokenComma),
            '.' => self.make_token(TokenType::TokenDot),
            '-' => self.make_token(TokenType::TokenMinus),
            '+' => self.make_token(TokenType::TokenPlus),
            '/' => self.make_token(TokenType::TokenSlash),
            '*' => self.make_token(TokenType::TokenStar),
            '!' => {
                let token_type = if self.match_token('=') {
                    TokenType::TokenBangEqual
                } else {
                    TokenType::TokenBang
                };
                self.make_token(token_type)
            }
            '=' => {
                let token_type = if self.match_token('=') {
                    TokenType::TokenEqualEqual
                } else {
                    TokenType::TokenEqual
                };
                self.make_token(token_type)
            }
            '<' => {
                let token_type = if self.match_token('=') {
                    TokenType::TokenLessEqual
                } else {
                    TokenType::TokenLess
                };
                self.make_token(token_type)
            }
            '>' => {
                let token_type = if self.match_token('=') {
                    TokenType::TokenGreaterEqual
                } else {
                    TokenType::TokenGreater
                };
                self.make_token(token_type)
            }
            '"' => self.string(),
            _ => self.error_token("Unexpected character.".to_string()),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
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
                    self.line += 1;
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
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth((self.current) as usize).unwrap();
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.chars().count() as i32 {
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
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.".to_string());
        }

        // The closing quote.
        self.advance();
        self.make_token(TokenType::TokenString)
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth((self.current) as usize).unwrap() != expected {
            return false;
        }

        self.current += 1;
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
        Token {
            token_type,
            lexeme: self.source[(self.start as usize)..(self.current as usize)].to_string(),
            line: self.line,
        }
    }

    fn error_token(&mut self, message: String) -> Token {
        Token {
            token_type: TokenType::TokenError,
            lexeme: message,
            line: self.line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn skip_whitespaces() {
        let source = " \r\t {".to_string();
        let mut scanner = Scanner::new(&source);

        let t = scanner.scan_token();
        assert_eq!(
            t.token_type,
            TokenType::TokenLeftBrace,
            "Expected to remove leading whitespace",
        );
    }

    #[rstest]
    #[case("{".to_string(), TokenType::TokenLeftBrace)]
    #[case("}".to_string(), TokenType::TokenRightBrace)]
    #[case("and".to_string(), TokenType::TokenAnd)]
    #[case("class".to_string(), TokenType::TokenClass)]
    #[case("!".to_string(), TokenType::TokenBang)]
    #[case("!=".to_string(), TokenType::TokenBangEqual)]
    #[case(",".to_string(), TokenType::TokenComma)]
    #[case(".".to_string(), TokenType::TokenDot)]
    #[case("else".to_string(), TokenType::TokenElse)]
    #[case("".to_string(), TokenType::TokenEof)]
    #[case("==".to_string(), TokenType::TokenEqualEqual)]
    #[case("false".to_string(), TokenType::TokenFalse)]
    #[case("for".to_string(), TokenType::TokenFor)]
    #[case("fun".to_string(), TokenType::TokenFun)]
    #[case(">".to_string(), TokenType::TokenGreater)]
    #[case(">=".to_string(), TokenType::TokenGreaterEqual)]
    #[case("if".to_string(), TokenType::TokenIf)]
    #[case("(".to_string(), TokenType::TokenLeftParen)]
    #[case(")".to_string(), TokenType::TokenRightParen)]
    #[case("<".to_string(), TokenType::TokenLess)]
    #[case("<=".to_string(), TokenType::TokenLessEqual)]
    #[case("-".to_string(), TokenType::TokenMinus)]
    #[case("nil".to_string(), TokenType::TokenNil)]
    #[case("123.1".to_string(), TokenType::TokenNumber)]
    #[case("or".to_string(), TokenType::TokenOr)]
    #[case("+".to_string(), TokenType::TokenPlus)]
    #[case("print".to_string(), TokenType::TokenPrint)]
    #[case("return".to_string(), TokenType::TokenReturn)]
    #[case(";".to_string(), TokenType::TokenSemicolon)]
    #[case("/".to_string(), TokenType::TokenSlash)]
    #[case("*".to_string(), TokenType::TokenStar)]
    #[case("super".to_string(), TokenType::TokenSuper)]
    #[case("this".to_string(), TokenType::TokenThis)]
    #[case("true".to_string(), TokenType::TokenTrue)]
    #[case("var".to_string(), TokenType::TokenVar)]
    #[case("while".to_string(), TokenType::TokenWhile)]
    #[case("\"hellow world\"".to_string(), TokenType::TokenString)]
    #[case("id".to_string(), TokenType::TokenIdentifier)]
    fn scan_tokens(#[case] input: String, #[case] expected_token: TokenType) {

        let mut scanner: Scanner = Scanner::new(&input);

        let token = scanner.scan_token();

        assert_eq!(
            token.token_type, expected_token,
            "Expected to scan {:?} token, got {:?}",
            expected_token, token.token_type
        );
    }
}
