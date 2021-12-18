use crate::chunk::Chunk;
use crate::common::{opcode_from_u8, OpCode};
use byteorder::{ByteOrder, LittleEndian};

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    print!("== {} ==\n", name);

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
        OpCode::OpReturn => return simple_instruction(String::from("OP_RETURN"), offset),
        OpCode::OpConstant => {
            return constant_instruction(String::from("OP_CONSTANT"), chunk, offset)
        }
        OpCode::OpConstantLong => {
            return long_constant_instruction(String::from("OP_CONSTANT_LONG"), chunk, offset)
        }
        OpCode::OpNil => return simple_instruction(String::from("OP_NIL"), offset),
        OpCode::OpTrue => return simple_instruction(String::from("OP_TRUE"), offset),
        OpCode::OpFalse => return simple_instruction(String::from("OP_FALSE"), offset),
        OpCode::OpGetLocal => {
            return byte_instruction(String::from("OP_GET_LOCAL"), chunk, offset);
        }
        OpCode::OpGetLocalLong => {
            return long_byte_instruction(String::from("OP_GET_LOCAL_LONG"), chunk, offset);
        }
        OpCode::OpSetLocal => {
            return byte_instruction(String::from("OP_SET_LOCAL"), chunk, offset);
        }
        OpCode::OpSetLocalLong => {
            return long_byte_instruction(String::from("OP_SET_LOCAL_LONG"), chunk, offset);
        }
        OpCode::OpGetGlobal => {
            return constant_instruction(String::from("OP_GET_GLOBAL"), chunk, offset)
        }
        OpCode::OpGetGlobalLong => {
            return long_constant_instruction(String::from("OP_GET_GLOBAL_LONG"), chunk, offset)
        }
        OpCode::OpDefineGlobal => {
            return constant_instruction(String::from("OP_DEFINE_GLOBAL"), chunk, offset)
        }
        OpCode::OpDefineGlobalLong => {
            return long_constant_instruction(String::from("OP_DEFINE_GLOBAL_LONG"), chunk, offset)
        }
        OpCode::OpSetGlobal => {
            return constant_instruction(String::from("OP_SET_GLOBAL"), chunk, offset)
        }
        OpCode::OpSetGlobalLong => {
            return long_constant_instruction(String::from("OP_SET_GLOBAL_LONG"), chunk, offset)
        }
        OpCode::OpEqual => return simple_instruction(String::from("OP_EQUAL"), offset),
        OpCode::OpGreater => return simple_instruction(String::from("OP_GREATER"), offset),
        OpCode::OpLess => return simple_instruction(String::from("OP_LESS"), offset),
        OpCode::OpAdd => return simple_instruction(String::from("OP_ADD"), offset),
        OpCode::OpSubstract => return simple_instruction(String::from("OP_SUBTRACT"), offset),
        OpCode::OpMultiply => return simple_instruction(String::from("OP_MULTIPLY"), offset),
        OpCode::OpDivide => return simple_instruction(String::from("OP_DIVIDE"), offset),
        OpCode::OpNot => return simple_instruction(String::from("OP_NOT"), offset),
        OpCode::OpNegate => return simple_instruction(String::from("OP_NEGATE"), offset),
        OpCode::OpPrint => return simple_instruction(String::from("OP_PRINT"), offset),
        OpCode::OpPop => return simple_instruction(String::from("OP_POP"), offset),
        _ => {
            println!("Unknown opcode {:?}\n", instruction);
            return offset + 1;
        }
    }
}

fn constant_instruction(name: String, chunk: &Chunk, offset: i32) -> i32 {
    let constant = chunk.code[(offset + 1) as usize];
    print!("{} {:#04} '", name, constant);
    chunk.constants.values[constant as usize].print_value();
    print!("'\n");
    offset + 2
}

fn long_constant_instruction(name: String, chunk: &Chunk, offset: i32) -> i32 {
    let mut buf = [0 as u8; 4];
    buf[..3].copy_from_slice(&chunk.code[(offset + 1) as usize..(offset + 4) as usize]);
    let constant = LittleEndian::read_u32(&buf);
    print!("{} {:#04} '", name, constant);
    chunk.constants.values[constant as usize].print_value();
    print!("'\n");
    offset + 4
}

fn simple_instruction(name: String, offset: i32) -> i32 {
    print!("{}\n", name);
    offset + 1
}

fn byte_instruction(name: String, chunk: &Chunk, offset: i32) -> i32 {
    let slot = chunk.code[(offset + 1) as usize];
    print!("{} {:#04} '", name, slot);
    return offset + 2;
}

fn long_byte_instruction(name: String, chunk: &Chunk, offset: i32) -> i32 {
    let mut buf = [0 as u8; 4];
    buf[..3].copy_from_slice(&chunk.code[(offset + 1) as usize..(offset + 4) as usize]);
    let slot = LittleEndian::read_u32(&buf);
    print!("{} {:#04} '", name, slot);
    return offset + 2;
}
