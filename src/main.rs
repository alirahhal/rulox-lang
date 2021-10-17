use vm::interpret;

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
    chunk.write_chunk(common::OpCode::OpNegate as u8, 123);

    chunk.write_chunk(common::OpCode::OpReturn as u8, 123);

    // debug::disassemble_chunk(&chunk, String::from("test chunk"));
    interpret(&chunk);
    chunk.free_chunk();
}
