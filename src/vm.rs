use std::collections::HashMap;

use byteorder::{BigEndian, ByteOrder, LittleEndian};

use crate::{
    chunk::Chunk,
    common::{opcode_from_u8, OpCode},
    compiler, debug,
    utils::stack::Stack,
    value::Value,
};

const DEBUG_TRACE_EXECUTION: bool = false;
const STACK_INITIAL_SIZE: usize = 256;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError, // Unknown,
}

pub struct VM<'a> {
    pub chunk: &'a Chunk,
    pub ip: &'a u8,

    pub stack: Stack,
    pub globals: HashMap<String, Value>,
}

pub fn interpret(source: &str) -> InterpretResult {
    let mut chunk = Chunk::new();

    if !compiler::compile(source, &mut chunk) {
        return InterpretResult::CompileError;
    }

    let mut vm = VM {
        chunk: &chunk,
        ip: &chunk.code[0],
        stack: Stack::new(Some(STACK_INITIAL_SIZE)),
        globals: HashMap::new(),
    };

    // self.chunk = &chunk;
    // self.ip = &self.chunk.code[0 as usize];

    let result = vm.run();

    chunk.free_chunk();
    result
}

impl<'a> VM<'a> {
    // pub fn new() -> Self {
    //     VM {
    //         chunk: &Chunk {
    //             ..Default::default()
    //         },
    //         ip: &0,
    //         stack: Stack::new(Some(STACK_INITIAL_SIZE)),
    //     }
    // }

    fn run(&mut self) -> InterpretResult {
        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("    ");
                self.stack.print_stack();
                debug::disassemble_instruction(self.chunk, unsafe {
                    (self.ip as *const u8).offset_from((&self.chunk.code[0]) as *const u8) as i32
                });
            }

            let instruction = self.read_byte();

            match opcode_from_u8(instruction).unwrap() {
                OpCode::OpConstant => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                }
                OpCode::OpConstantLong => {
                    let constant = self.read_long_constant();
                    self.stack.push(constant);
                }
                OpCode::OpNil => {
                    self.stack.push(Value::new_nil());
                }
                OpCode::OpTrue => {
                    self.stack.push(Value::new_bool(true));
                }
                OpCode::OpFalse => {
                    self.stack.push(Value::new_bool(false));
                }
                OpCode::OpPop => {
                    self.stack.pop();
                }
                OpCode::OpGetLocal => {
                    let slot = self.read_byte();
                    self.stack.push(self.stack.get_at(slot as usize).clone());
                }
                OpCode::OpGetLocalLong => {
                    let slot = self.read_long();
                    self.stack.push(self.stack.get_at(slot as usize).clone());
                }
                OpCode::OpSetLocal => {
                    let slot = self.read_byte();
                    self.stack.set_at(slot as usize, self.peek(0).clone());
                }
                OpCode::OpSetLocalLong => {
                    let slot = self.read_long();
                    self.stack.push(self.stack.get_at(slot as usize).clone());
                }
                OpCode::OpGetGlobal => {
                    let v = self.read_constant();
                    let name = v.as_string();
                    let value = match self.globals.get(name) {
                        Some(v) => v,
                        None => {
                            self.runtime_error(format!("Undefined variable '{}'.", "name"));
                            return InterpretResult::RuntimeError;
                        }
                    };

                    self.stack.push(value.clone());
                }
                OpCode::OpGetGlobalLong => {
                    let v = self.read_long_constant();
                    let name = v.as_string();
                    let value = match self.globals.get(name) {
                        Some(val) => val,
                        None => {
                            self.runtime_error(format!("Undefined variable '{}'.", name));
                            return InterpretResult::RuntimeError;
                        }
                    };

                    self.stack.push(value.clone());
                }
                OpCode::OpDefineGlobal => {
                    let v = self.read_constant();
                    let name = v.as_string().to_owned();
                    self.globals.insert(name, self.peek(0).clone());

                    self.stack.pop();
                }
                OpCode::OpDefineGlobalLong => {
                    let v = self.read_long_constant();
                    let name = v.as_string().to_owned();
                    self.globals.insert(name, self.peek(0).clone());

                    self.stack.pop();
                }
                OpCode::OpSetGlobal => {
                    let name = self.read_constant().as_string().to_owned();
                    if !self.globals.contains_key(&name) {
                        self.runtime_error(format!("Undefined variable '{}'.", name));
                        return InterpretResult::RuntimeError;
                    }

                    self.globals.insert(name, self.peek(0).clone());
                }
                OpCode::OpSetGlobalLong => {
                    let name = self.read_long_constant().as_string().to_owned();
                    if !self.globals.contains_key(&name) {
                        self.runtime_error(format!("Undefined variable '{}'.", name));
                        return InterpretResult::RuntimeError;
                    }

                    self.globals.insert(name, self.peek(0).clone());
                }
                OpCode::OpEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::new_bool(a.values_equal(&b)));
                }
                OpCode::OpGreater => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return InterpretResult::RuntimeError;
                    }

                    self.binary_op(|a, b| Value::new_bool(a.as_number() > b.as_number()));
                }
                OpCode::OpLess => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return InterpretResult::RuntimeError;
                    }

                    self.binary_op(|a, b| Value::new_bool(a.as_number() < b.as_number()));
                }
                OpCode::OpAdd => {
                    if self.peek(0).is_string() && self.peek(1).is_string() {
                        self.concatenate()
                    } else if self.peek(0).is_number() && self.peek(1).is_number() {
                        self.binary_op(|a, b| Value::new_number(a.as_number() + b.as_number()));
                    } else {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return InterpretResult::RuntimeError;
                    }
                }
                OpCode::OpSubtract => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return InterpretResult::RuntimeError;
                    }

                    self.binary_op(|a, b| Value::new_number(a.as_number() - b.as_number()));
                }
                OpCode::OpMultiply => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return InterpretResult::RuntimeError;
                    }

                    self.binary_op(|a, b| Value::new_number(a.as_number() * b.as_number()));
                }
                OpCode::OpDivide => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return InterpretResult::RuntimeError;
                    }

                    self.binary_op(|a, b| Value::new_number(a.as_number() / b.as_number()));
                }
                OpCode::OpNot => {
                    let popped = self.stack.pop().unwrap();
                    self.stack.push(Value::new_bool(popped.is_falsey()));
                }
                OpCode::OpNegate => {
                    if !self.peek(0).is_number() {
                        self.runtime_error("Operand must be a number.".to_string());
                        return InterpretResult::RuntimeError;
                    }
                    let value_to_negate = self.stack.pop().unwrap().as_number();
                    self.stack.push(Value::new_number(-value_to_negate));
                }
                OpCode::OpPrint => {
                    self.stack.pop().unwrap().print_value();
                    println!();
                }
                OpCode::OpJumpIfFalse => {
                    let offset = self.read_short();
                    if self.peek(0).is_falsey() {
                        let ptr = self.ip as *const u8;
                        self.ip = unsafe { ptr.offset(offset as isize).as_ref().unwrap() };
                    }
                }
                OpCode::OpJump => {
                    let offset = self.read_short();
                    let ptr = self.ip as *const u8;
                    self.ip = unsafe { ptr.offset(offset as isize).as_ref().unwrap() };
                }
                OpCode::OpLoop => {
                    let offset = self.read_short();
                    let ptr = self.ip as *const u8;
                    self.ip = unsafe { ptr.offset(-(offset as isize)).as_ref().unwrap() };
                }
                OpCode::OpReturn => {
                    // Exit interpreter.
                    return InterpretResult::Ok;
                }
                _ => panic!("Unknown opcode {:?}\n", instruction),
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

    fn peek(&self, distance: usize) -> &Value {
        self.stack.peek(distance)
    }

    fn concatenate(&mut self) {
        let b_option = self.stack.pop().unwrap();
        let a_option = self.stack.pop().unwrap();
        let b = b_option.as_string();
        let a = a_option.as_string();
        let mut s = String::with_capacity(a.len() + b.len());
        s.push_str(a);
        s.push_str(b);

        let value = Value::new_obj_string(s);

        self.stack.push(value);
    }

    fn runtime_error(&self, message: String) {
        println!("{}", message);
    }

    fn read_byte(&mut self) -> u8 {
        unsafe { self.read_byte_unsafe() }
    }

    unsafe fn read_byte_unsafe(&mut self) -> u8 {
        let current_byte = *self.ip;

        let ptr = self.ip as *const u8;
        self.ip = ptr.offset(1).as_ref().unwrap();

        current_byte
    }

    fn read_short(&mut self) -> u16 {
        let mut buf = [0_u8; 4];
        for i in 0..2 {
            buf[i] = self.read_byte();
        }
        BigEndian::read_u16(&buf)
    }

    fn read_long(&mut self) -> u32 {
        let mut buf = [0_u8; 4];
        for i in 0..3 {
            buf[i] = self.read_byte();
        }
        LittleEndian::read_u32(&buf)
    }

    fn read_constant(&mut self) -> Value {
        self.chunk.constants.values[self.read_byte() as usize].clone()
    }

    fn read_long_constant(&mut self) -> Value {
        let mut buf = [0_u8; 4];
        for i in 0..3 {
            buf[i] = self.read_byte();
        }
        let constant_address = LittleEndian::read_u32(&buf);
        self.chunk.constants.values[constant_address as usize].clone()
    }
}
