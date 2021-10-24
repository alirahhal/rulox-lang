mod chunk;
mod common;
mod compiler;
mod debug;
mod scanner;
mod utils;
mod value;
mod vm;

use std::{
    env, fs,
    io::{self, Write},
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args.len());

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: clox [path]");
        process::exit(64);
    }
}

fn run_file(path: &String) {
    let source = fs::read_to_string(path).expect("Something went wrong reading the file");

    let result = vm::VM::interpret(&source);

    match result {
        vm::InterpretResult::InterpretCompileError => process::exit(65),
        vm::InterpretResult::InterpretRuntimeError => process::exit(70),
        _ => (),
    }
}

fn repl() {
    let mut line = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        if io::stdin().read_line(&mut line).unwrap_or(0) == 0 {
            println!();
            break;
        }

        vm::VM::interpret(&line);
        line.clear();
    }
}
