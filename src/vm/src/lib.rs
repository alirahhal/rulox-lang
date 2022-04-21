pub mod debug;
mod stack;
pub mod vm;

use common::chunk::Chunk;

use vm::RunResult;

pub use vm::RunResult as InterpretResult;
pub use vm::VM as VM;

pub fn run(chunk: &Chunk) -> RunResult {
    let mut vm = vm::VM::new(chunk);

    let result = vm.run();

    result
}
