use std::{
    env, fs,
    io::{self, Write},
    process,
};

use vm::{InterpretResult};

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Wrong number of arguments");
        process::exit(64);
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("Something went wrong reading the file");

    let chunk = compiler::compile(&source);
    if let Err(_) = chunk {
        println!("Failed");
        return;
    }

    let result = vm::run(&chunk.unwrap());

    match result {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        _ => (),
    }
}

fn repl() {
    let mut line = String::new();
    let mut vm = vm::VM::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        if io::stdin().read_line(&mut line).unwrap_or(0) == 0 {
            println!();
            break;
        }

        let chunk = compiler::compile(&line);
        if let Err(_) = chunk {
            println!("Failed");
            return;
        }

        let result = vm.run(&chunk.unwrap());

        match result {
            InterpretResult::CompileError => process::exit(65),
            InterpretResult::RuntimeError => process::exit(70),
            _ => (),
        }

        line.clear();
    }
}
