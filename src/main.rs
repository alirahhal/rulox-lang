mod chunk;
mod common;
mod debug;
mod value;
mod vm;
mod stack;

fn main() {
    let mut chunk = chunk::Chunk {
        ..Default::default()
    };
    chunk.write_constant(1.2, 123);
    chunk.write_constant(3.4, 123);

    chunk.write_chunk(common::OpCode::OpAdd as u8, 123);

    chunk.write_constant(5.6, 123);

    chunk.write_chunk(common::OpCode::OpDivide as u8, 123);
    chunk.write_chunk(common::OpCode::OpNegate as u8, 123);

    chunk.write_chunk(common::OpCode::OpReturn as u8, 123);

    vm::VM::interpret(&chunk);
    chunk.free_chunk();
}
