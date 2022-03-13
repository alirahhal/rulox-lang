pub mod stack;
pub mod vm;
pub mod debug;

use std::collections::HashMap;

use common::chunk::Chunk;
use stack::Stack;
use vm::{RunResult, VM, STACK_INITIAL_SIZE};

pub fn run(chunk: &Chunk) -> RunResult {
    let mut vm = VM {
        chunk: &chunk,
        ip: &chunk.code[0],
        stack: Stack::new(Some(STACK_INITIAL_SIZE)),
        globals: HashMap::new(),
    };

    let result = vm.run();

    result
}