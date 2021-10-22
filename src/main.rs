mod chunk;
mod common;
mod debug;
mod utils;
mod value;
mod vm;

fn main() {
    let mut chunk = chunk::Chunk::new();

    chunk.write_constant(1 as f64, 123);
    chunk.write_constant(3 as f64, 123);

    chunk.write_chunk(common::OpCode::OpAdd as u8, 123);

    chunk.write_constant(5 as f64, 123);

    chunk.write_chunk(common::OpCode::OpDivide as u8, 123);
    chunk.write_chunk(common::OpCode::OpNegate as u8, 123);

    chunk.write_chunk(common::OpCode::OpReturn as u8, 123);

    vm::VM::interpret(&chunk);
    chunk.free_chunk();
}
