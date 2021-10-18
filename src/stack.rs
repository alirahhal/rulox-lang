use crate::value::Value;

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

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    pub fn print_stack(&mut self) {
        println!("{:?}", self.stack);
    }
}
