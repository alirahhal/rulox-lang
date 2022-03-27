use common::chunk::Chunk;
use parser::Parser;
use scanner::scanner::Scanner;

mod scanner;
mod compiler;
mod parser;

pub fn compile(source: &str) -> Result<Chunk, ()> {
    let mut scanner = Scanner::new(source);
    let mut chunk = Chunk::new();
    let mut parser = Parser::new(&mut scanner, &mut chunk);

    parser.parse();

    Ok(chunk)
    // !parser.had_error
}