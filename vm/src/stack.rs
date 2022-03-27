use common::{value::Value};

#[derive(Default)]
pub struct Stack {
    stack: Vec<Value>,
}

impl Stack {
    pub fn new(initial_capacity: Option<usize>) -> Self {
        match initial_capacity {
            Some(cap) => Stack {
                stack: Vec::with_capacity(cap),
            },
            None => Stack { stack: Vec::new() },
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    pub fn get_at(&self, index: usize) -> &Value {
        &self.stack[index]
    }

    pub fn set_at(&mut self, index: usize, value: Value) {
        self.stack[index] = value;
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn peek(&self, distance: usize) -> &Value {
        &self.stack[(self.stack.len() - 1 - distance) as usize]
    }

    pub fn print_stack(&self) {
        print!("[ ");
        for v in self.stack.iter().enumerate() {
            if v.0 == self.stack.len() - 1 {
                v.1.print_value();
            } else {
                v.1.print_value();
                print!(", ");
            }
        }
        println!(" ]");
    }
}
