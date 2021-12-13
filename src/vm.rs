use std::{collections::HashMap, rc::Rc};

use byteorder::{ByteOrder, LittleEndian};

use crate::{
    chunk::Chunk,
    common::{opcode_from_u8, OpCode},
    compiler, debug,
    object::{Obj, ObjString, ObjType},
    utils::stack::Stack,
    value::Value,
};

const DEBUG_TRACE_EXECUTION: bool = true;
const STACK_INITIAL_SIZE: usize = 256;

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError, // Unknown,
}

pub struct VM<'a> {
    pub chunk: &'a Chunk,
    pub ip: &'a u8,

    pub stack: Stack,
    pub globals: HashMap<String, Value>,
}

pub fn interpret(source: &String) -> InterpretResult {
    let mut chunk = Chunk::new();

    if !compiler::compile(source, &mut chunk) {
        return InterpretResult::InterpretCompileError;
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
    return result;
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
                OpCode::OpGetGlobal => {
                    let name = self.read_constant().as_rust_string();
                    let value = match self.globals.get(&name) {
                        Some(val) => val,
                        None => {
                            self.runtime_error(format!("Undefined variable '{}'.", name));
                            return InterpretResult::InterpretRuntimeError;
                        }
                    };

                    self.stack.push(value.clone());
                }
                OpCode::OpGetGlobalLong => {
                    let name = self.read_long_constant().as_rust_string();
                    let value = match self.globals.get(&name) {
                        Some(val) => val,
                        None => {
                            self.runtime_error(format!("Undefined variable '{}'.", name));
                            return InterpretResult::InterpretRuntimeError;
                        }
                    };

                    self.stack.push(value.clone());
                }
                OpCode::OpDefineGlobal => {
                    let name = self.read_constant().as_rust_string();
                    self.globals.insert(name.to_owned(), self.peek(0).clone());

                    self.stack.pop();
                }
                OpCode::OpDefineGlobalLong => {
                    let name = self.read_long_constant().as_rust_string();
                    self.globals.insert(name.to_owned(), self.peek(0).clone());

                    self.stack.pop();
                }
                OpCode::OpSetGlobal => {
                    let name = self.read_constant().as_rust_string();
                    if !self.globals.contains_key(&name) {
                        self.runtime_error(format!("Undefined variable '{}'.", name));
                        return InterpretResult::InterpretRuntimeError;
                    }

                    self.globals.insert(name, self.peek(0).clone());
                }
                OpCode::OpSetGlobalLong => {
                    let name = self.read_long_constant().as_rust_string();
                    if !self.globals.contains_key(&name) {
                        self.runtime_error(format!("Undefined variable '{}'.", name));
                        return InterpretResult::InterpretRuntimeError;
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
                        return InterpretResult::InterpretRuntimeError;
                    }

                    self.binary_op(|a, b| Value::new_bool(a.as_number() > b.as_number()));
                }
                OpCode::OpLess => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return InterpretResult::InterpretRuntimeError;
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
                        return InterpretResult::InterpretRuntimeError;
                    }
                }
                OpCode::OpSubstract => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return InterpretResult::InterpretRuntimeError;
                    }

                    self.binary_op(|a, b| Value::new_number(a.as_number() - b.as_number()));
                }
                OpCode::OpMultiply => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return InterpretResult::InterpretRuntimeError;
                    }

                    self.binary_op(|a, b| Value::new_number(a.as_number() * b.as_number()));
                }
                OpCode::OpDivide => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.".to_string());
                        return InterpretResult::InterpretRuntimeError;
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
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let value_to_negate = self.stack.pop().unwrap().as_number();
                    self.stack.push(Value::new_number(-value_to_negate));
                }
                OpCode::OpPrint => {
                    self.stack.pop().unwrap().print_value();
                    println!();
                }
                OpCode::OpReturn => {
                    // Exit interpreter.
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

    fn peek(&self, distance: usize) -> &Value {
        self.stack.peek(distance)
    }

    fn concatenate(&mut self) {
        let b = self.stack.pop().unwrap().as_rust_string();
        let a = self.stack.pop().unwrap().as_rust_string();

        let value = Value::new_obj(Rc::new(ObjString::new(format!("{}{}", a, b))));

        self.stack.push(value.clone());
    }

    fn runtime_error(&self, message: String) {
        println!("{}", message);
    }

    unsafe fn read_byte(&mut self) -> u8 {
        let current_byte = *self.ip;

        let ptr = self.ip as *const u8;
        self.ip = ptr.offset(1).as_ref().unwrap();

        current_byte
    }

    fn read_constant(&mut self) -> Value {
        self.chunk.constants.values[unsafe { self.read_byte() } as usize].clone()
    }

    fn read_long_constant(&mut self) -> Value {
        let mut buf = [0 as u8; 4];
        for i in 0..3 {
            unsafe {
                buf[i] = self.read_byte();
            }
        }
        let constant_address = LittleEndian::read_u32(&buf);
        self.chunk.constants.values[constant_address as usize].clone()
    }
}
