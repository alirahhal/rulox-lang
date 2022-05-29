use std::collections::HashMap;

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use common::{chunk::Chunk, opcode::OpCode, value::Value};

use crate::{debug, stack::Stack};

const DEBUG_TRACE_EXECUTION: bool = false;
pub const STACK_INITIAL_SIZE: usize = 256;

pub enum RunResult {
    Ok,
    CompileError,
    RuntimeError, // Unknown,
}

pub struct VM {
    pub stack: Stack,
    pub globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Stack::new(Some(STACK_INITIAL_SIZE)),
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self, chunk: &Chunk) -> RunResult {
        let mut ip = &chunk.code[0];
        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("    ");
                self.stack.print_stack();
                debug::disassemble_instruction(chunk, unsafe {
                    (ip as *const u8).offset_from((&chunk.code[0]) as *const u8) as i32
                });
            }

            let instruction = self.read_byte(&mut ip);

            match OpCode::try_from(instruction).unwrap() {
                OpCode::OpConstant => {
                    let constant = self.read_constant(&mut ip, chunk);
                    self.stack.push(constant);
                }
                OpCode::OpConstantLong => {
                    let constant = self.read_long_constant(&mut ip, chunk);
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
                    let slot = self.read_byte(&mut ip);
                    self.stack.push(self.stack.get_at(slot as usize).clone());
                }
                OpCode::OpGetLocalLong => {
                    let slot = self.read_long(&mut ip);
                    self.stack.push(self.stack.get_at(slot as usize).clone());
                }
                OpCode::OpSetLocal => {
                    let slot = self.read_byte(&mut ip);
                    self.stack.set_at(slot as usize, self.peek(0).clone());
                }
                OpCode::OpSetLocalLong => {
                    let slot = self.read_long(&mut ip);
                    self.stack.push(self.stack.get_at(slot as usize).clone());
                }
                OpCode::OpGetGlobal => {
                    let v = self.read_constant(&mut ip, chunk);
                    let name = v.as_string();
                    let value = match self.globals.get(name) {
                        Some(v) => v,
                        None => {
                            self.runtime_error(format!("Undefined variable '{}'.", "name"));
                            return RunResult::RuntimeError;
                        }
                    };

                    self.stack.push(value.clone());
                }
                OpCode::OpGetGlobalLong => {
                    let v = self.read_long_constant(&mut ip, chunk);
                    let name = v.as_string();
                    let value = match self.globals.get(name) {
                        Some(val) => val,
                        None => {
                            self.runtime_error(format!("Undefined variable '{}'.", name));
                            return RunResult::RuntimeError;
                        }
                    };

                    self.stack.push(value.clone());
                }
                OpCode::OpDefineGlobal => {
                    let v = self.read_constant(&mut ip, chunk);
                    let name = v.as_string().to_owned();
                    self.globals.insert(name, self.peek(0).clone());

                    self.stack.pop();
                }
                OpCode::OpDefineGlobalLong => {
                    let v = self.read_long_constant(&mut ip, chunk);
                    let name = v.as_string().to_owned();
                    self.globals.insert(name, self.peek(0).clone());

                    self.stack.pop();
                }
                OpCode::OpSetGlobal => {
                    let name = self.read_constant(&mut ip, chunk).as_string().to_owned();
                    if !self.globals.contains_key(&name) {
                        self.runtime_error(format!("Undefined variable '{}'.", name));
                        return RunResult::RuntimeError;
                    }

                    self.globals.insert(name, self.peek(0).clone());
                }
                OpCode::OpSetGlobalLong => {
                    let name = self.read_long_constant(&mut ip, chunk).as_string().to_owned();
                    if !self.globals.contains_key(&name) {
                        self.runtime_error(format!("Undefined variable '{}'.", name));
                        return RunResult::RuntimeError;
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
                        return RunResult::RuntimeError;
                    }

                    self.binary_op(|a, b| Value::new_bool(a.as_number() > b.as_number()));
                }
                OpCode::OpLess => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return RunResult::RuntimeError;
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
                        return RunResult::RuntimeError;
                    }
                }
                OpCode::OpSubtract => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return RunResult::RuntimeError;
                    }

                    self.binary_op(|a, b| Value::new_number(a.as_number() - b.as_number()));
                }
                OpCode::OpMultiply => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return RunResult::RuntimeError;
                    }

                    self.binary_op(|a, b| Value::new_number(a.as_number() * b.as_number()));
                }
                OpCode::OpDivide => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return RunResult::RuntimeError;
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
                        return RunResult::RuntimeError;
                    }
                    let value_to_negate = self.stack.pop().unwrap().as_number();
                    self.stack.push(Value::new_number(-value_to_negate));
                }
                OpCode::OpPrint => {
                    self.stack.pop().unwrap().print_value();
                    println!();
                }
                OpCode::OpJumpIfFalse => {
                    let offset = self.read_short(&mut ip);
                    if self.peek(0).is_falsey() {
                        let ptr = ip as *const u8;
                        ip = unsafe { &mut ptr.offset(offset as isize).as_ref().unwrap() };
                    }
                }
                OpCode::OpJump => {
                    let offset = self.read_short(&mut ip);
                    let ptr = ip as *const u8;
                    ip = unsafe { ptr.offset(offset as isize).as_ref().unwrap() };
                }
                OpCode::OpLoop => {
                    let offset = self.read_short(&mut ip);
                    let ptr = ip as *const u8;
                    ip = unsafe { ptr.offset(-(offset as isize)).as_ref().unwrap() };
                }
                OpCode::OpReturn => {
                    // Exit interpreter.
                    return RunResult::Ok;
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

    fn read_byte(&mut self, ip: &mut &u8) -> u8 {
        unsafe { self.read_byte_unsafe(ip) }
    }

    unsafe fn read_byte_unsafe(&mut self, ip: &mut &u8) -> u8 {
        let current_byte = *ip;

        let ptr = *ip as *const u8;
        *ip = &*ptr.offset(1).as_ref().unwrap();

        *current_byte
    }

    fn read_short(&mut self, ip: &mut &u8) -> u16 {
        let mut buf = [0_u8; 4];
        for i in 0..2 {
            buf[i] = self.read_byte(ip);
        }
        BigEndian::read_u16(&buf)
    }

    fn read_long(&mut self, ip: &mut &u8) -> u32 {
        let mut buf = [0_u8; 4];
        for i in 0..3 {
            buf[i] = self.read_byte(ip);
        }
        LittleEndian::read_u32(&buf)
    }

    fn read_constant(&mut self, ip: &mut &u8, chunk: &Chunk) -> Value {
        chunk.constants.values[self.read_byte(ip) as usize].clone()
    }

    fn read_long_constant(&mut self, ip: &mut &u8, chunk: &Chunk) -> Value {
        let mut buf = [0_u8; 4];
        for i in 0..3 {
            buf[i] = self.read_byte(ip);
        }
        let constant_address = LittleEndian::read_u32(&buf);
        chunk.constants.values[constant_address as usize].clone()
    }
}
