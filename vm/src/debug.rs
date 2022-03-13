
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use common::chunk::{Chunk, OpCode, opcode_from_u8};

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("== {} ==", name);

    let mut offset: i32 = 0;
    while offset < chunk.code.len() as i32 {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: i32) -> i32 {
    print!("{:#04} ", offset);
    if offset > 0 && chunk.lines[offset as usize] == chunk.lines[(offset - 1) as usize] {
        print!("    | ");
    } else {
        print!("{:#4} ", chunk.lines[offset as usize]);
    }

    let instruction = chunk.code[offset as usize];
    match opcode_from_u8(instruction).unwrap_or_default() {
        OpCode::OpReturn => simple_instruction(String::from("OP_RETURN"), offset),
        OpCode::OpConstant => constant_instruction(String::from("OP_CONSTANT"), chunk, offset),
        OpCode::OpConstantLong => {
            long_constant_instruction(String::from("OP_CONSTANT_LONG"), chunk, offset)
        }
        OpCode::OpNil => simple_instruction(String::from("OP_NIL"), offset),
        OpCode::OpTrue => simple_instruction(String::from("OP_TRUE"), offset),
        OpCode::OpFalse => simple_instruction(String::from("OP_FALSE"), offset),
        OpCode::OpGetLocal => byte_instruction(String::from("OP_GET_LOCAL"), chunk, offset),
        OpCode::OpGetLocalLong => {
            long_byte_instruction(String::from("OP_GET_LOCAL_LONG"), chunk, offset)
        }
        OpCode::OpSetLocal => byte_instruction(String::from("OP_SET_LOCAL"), chunk, offset),
        OpCode::OpSetLocalLong => {
            long_byte_instruction(String::from("OP_SET_LOCAL_LONG"), chunk, offset)
        }
        OpCode::OpGetGlobal => constant_instruction(String::from("OP_GET_GLOBAL"), chunk, offset),
        OpCode::OpGetGlobalLong => {
            long_constant_instruction(String::from("OP_GET_GLOBAL_LONG"), chunk, offset)
        }
        OpCode::OpDefineGlobal => {
            constant_instruction(String::from("OP_DEFINE_GLOBAL"), chunk, offset)
        }
        OpCode::OpDefineGlobalLong => {
            long_constant_instruction(String::from("OP_DEFINE_GLOBAL_LONG"), chunk, offset)
        }
        OpCode::OpSetGlobal => constant_instruction(String::from("OP_SET_GLOBAL"), chunk, offset),
        OpCode::OpSetGlobalLong => {
            long_constant_instruction(String::from("OP_SET_GLOBAL_LONG"), chunk, offset)
        }
        OpCode::OpEqual => simple_instruction(String::from("OP_EQUAL"), offset),
        OpCode::OpGreater => simple_instruction(String::from("OP_GREATER"), offset),
        OpCode::OpLess => simple_instruction(String::from("OP_LESS"), offset),
        OpCode::OpAdd => simple_instruction(String::from("OP_ADD"), offset),
        OpCode::OpSubtract => simple_instruction(String::from("OP_SUBTRACT"), offset),
        OpCode::OpMultiply => simple_instruction(String::from("OP_MULTIPLY"), offset),
        OpCode::OpDivide => simple_instruction(String::from("OP_DIVIDE"), offset),
        OpCode::OpNot => simple_instruction(String::from("OP_NOT"), offset),
        OpCode::OpNegate => simple_instruction(String::from("OP_NEGATE"), offset),
        OpCode::OpPrint => simple_instruction(String::from("OP_PRINT"), offset),
        OpCode::OpJumpIfFalse => {
            jump_instruction(String::from("OP_JUMP_IF_FALSE"), 1, chunk, offset)
        }
        OpCode::OpJump => jump_instruction(String::from("OP_JUMP"), 1, chunk, offset),
        OpCode::OpLoop => jump_instruction(String::from("OP_LOOP"), -1, chunk, offset),
        OpCode::OpPop => simple_instruction(String::from("OP_POP"), offset),
        _ => {
            println!("Unknown opcode {:?}\n", instruction);
            offset + 1
        }
    }
}

fn constant_instruction(name: String, chunk: &Chunk, offset: i32) -> i32 {
    let constant = chunk.code[(offset + 1) as usize];
    print!("{} {:#04} '", name, constant);
    chunk.constants.values[constant as usize].print_value();
    println!("'");
    offset + 2
}

fn long_constant_instruction(name: String, chunk: &Chunk, offset: i32) -> i32 {
    let mut buf = [0_u8; 4];
    buf[..3].copy_from_slice(&chunk.code[(offset + 1) as usize..(offset + 4) as usize]);
    let constant = LittleEndian::read_u32(&buf);
    print!("{} {:#04} '", name, constant);
    chunk.constants.values[constant as usize].print_value();
    println!("'");
    offset + 4
}

fn simple_instruction(name: String, offset: i32) -> i32 {
    println!("{}", name);
    offset + 1
}

fn byte_instruction(name: String, chunk: &Chunk, offset: i32) -> i32 {
    let slot = chunk.code[(offset + 1) as usize];
    print!("{} {:#04} '", name, slot);
    offset + 2
}

fn long_byte_instruction(name: String, chunk: &Chunk, offset: i32) -> i32 {
    let mut buf = [0_u8; 4];
    buf[..3].copy_from_slice(&chunk.code[(offset + 1) as usize..(offset + 4) as usize]);
    let slot = LittleEndian::read_u32(&buf);
    print!("{} {:#04} '", name, slot);
    offset + 4
}

fn jump_instruction(name: String, sign: i32, chunk: &Chunk, offset: i32) -> i32 {
    let mut buf = [0_u8; 4];
    buf[..2].copy_from_slice(&chunk.code[(offset + 1) as usize..(offset + 3) as usize]);
    let jump = BigEndian::read_u16(&buf);
    println!(
        "{} {:#04} -> {}",
        name,
        offset,
        offset + 3 + sign * (jump as i32)
    );
    offset + 3
}
