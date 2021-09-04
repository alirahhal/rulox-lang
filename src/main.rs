mod chunk;
mod common;
mod debug;
mod value;

fn main() {
    let mut chunk = chunk::Chunk {
        ..Default::default()
    };
    chunk.write_constant(1.2, 123);
    chunk.write_constant(5.3, 124);
    chunk.write_constant(5.4, 124);

    chunk.write_chunk(chunk::OpCode::OpReturn as u8, 123);

    debug::disassemble_chunk(&chunk, String::from("test chunk"));
    chunk.free_chunk();
}
