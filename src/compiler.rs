use crate::{
    common::TokenType,
    scanner::{self, Scanner},
};

pub fn compile(source: &String) {
    let mut scanner = Scanner::init_scanner(source);
    let mut line = -1;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:#04} ", token.line);
            line = token.line;
        } else {
            print!("    | ");
        }
        println!("'{}'", token.lexeme);

        if token.token_type == TokenType::TokenEof {
            break;
        }
    }
}
