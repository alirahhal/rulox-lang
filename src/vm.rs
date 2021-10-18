use byteorder::{ByteOrder, LittleEndian};

use crate::{
    chunk::Chunk,
    common::{opcode_from_u8, OpCode},
    debug,
    stack::Stack,
    value::{print_value, Value},
};

const DEBUG_TRACE_EXECUTION: bool = true;
const STACK_INITIAL_SIZE: usize = 256;

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
    Unknown,
}

pub struct VM<'a> {
    pub chunk: &'a Chunk,
    pub ip: &'a u8,

    pub stack: Stack,
}

impl<'a> VM<'a> {
    pub fn interpret(chunk: &Chunk) -> InterpretResult {
        let mut vm = VM {
            chunk,
            ip: &chunk.code[0],
            stack: Stack::new(Some(STACK_INITIAL_SIZE)),
        };
        vm.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("    ");
                self.stack.print_stack();
                debug::disassemble_instruction(self.chunk, unsafe {
                    (self.ip as *const u8).offset_from((&self.chunk.code[0]) as *const u8) as i32
                });
            }

            let instruction: u8;
            unsafe {
                instruction = self.read_byte();
            }

            match opcode_from_u8(instruction).unwrap_or_default() {
                OpCode::OpConstant => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                }
                OpCode::OpConstantLong => {
                    let constant = self.read_long_constant();
                    self.stack.push(constant);
                }
                OpCode::OpAdd => {
                    self.binary_op(|a, b| a + b);
                }
                OpCode::OpSubstract => {
                    self.binary_op(|a, b| a - b);
                }
                OpCode::OpMultiply => {
                    self.binary_op(|a, b| a * b);
                }
                OpCode::OpDivide => {
                    self.binary_op(|a, b| a / b);
                }
                OpCode::OpNegate => {
                    let value_to_negate = self.stack.pop().unwrap();
                    self.stack.push(-value_to_negate);
                }
                OpCode::OpReturn => {
                    print_value(self.stack.pop().unwrap());
                    println!();
                    return InterpretResult::InterpretOk;
                }
                _ => {
                    panic!("Unknown opcode {:?}\n", instruction);
                }
            }
        }
    }

    fn binary_op(&mut self, callback: fn(Value, Value) -> Value) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(callback(a, b));
    }

    fn reset_stack(&mut self) {
        self.stack = Stack::new(Some(STACK_INITIAL_SIZE));
    }

    unsafe fn read_byte(&mut self) -> u8 {
        let current_byte = *self.ip;

        let ptr = self.ip as *const u8;
        self.ip = ptr.offset(1).as_ref().unwrap();

        current_byte
    }

    fn read_constant(&mut self) -> Value {
        self.chunk.constants.values[unsafe { self.read_byte() } as usize]
    }

    fn read_long_constant(&mut self) -> Value {
        let mut buf = [0 as u8; 4];
        for i in 0..3 {
            unsafe {
                buf[i] = self.read_byte();
            }
        }
        let constant_address = LittleEndian::read_u32(&buf);
        self.chunk.constants.values[constant_address as usize]
    }
}
