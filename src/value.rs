use std::{any::Any, fmt, rc::Rc};

use crate::common::ValueType;

#[derive(Debug)]
pub struct Value {
    pub value_type: ValueType,
    pub value: Rc<dyn Any>,
}

impl Clone for Value {
    fn clone(&self) -> Self {
        Self {
            value_type: self.value_type,
            value: Rc::clone(&self.value),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}

impl Value {
    pub fn new_bool(val: bool) -> Self {
        Value {
            value_type: ValueType::ValBool,
            value: Rc::new(val),
        }
    }

    pub fn new_nil() -> Self {
        Value {
            value_type: ValueType::ValNil,
            value: Rc::new(0),
        }
    }

    pub fn new_number(val: i64) -> Self {
        Value {
            value_type: ValueType::ValNumber,
            value: Rc::new(val),
        }
    }

    pub fn as_bool(&self) -> bool {
        *self.value.downcast_ref::<bool>().unwrap()
    }

    pub fn as_number(&self) -> i64 {
        *self.value.downcast_ref::<i64>().unwrap()
    }

    pub fn is_bool(&self) -> bool {
        if self.value_type == ValueType::ValBool {
            true
        } else {
            false
        }
    }

    pub fn is_nil(&self) -> bool {
        if self.value_type == ValueType::ValNil {
            true
        } else {
            false
        }
    }

    pub fn is_number(&self) -> bool {
        if self.value_type == ValueType::ValNumber {
            true
        } else {
            false
        }
    }

    pub fn is_falsey(&self) -> bool {
        self.is_nil() || (self.is_bool() && !self.as_bool())
    }

    pub fn print_value(&self) {
        match self.value_type {
            ValueType::ValBool => {
                if self.as_bool() {
                    print!("true");
                } else {
                    print!("false");
                }
            }
            ValueType::ValNil => print!("nil"),
            ValueType::ValNumber => print!("{}", self.as_number()),
        }
    }

    pub fn values_equal(&self, other: &Value) -> bool {
        if self.value_type != other.value_type {
            return false;
        }

        match self.value_type {
            ValueType::ValBool => self.as_bool() == other.as_bool(),
            ValueType::ValNil => true,
            ValueType::ValNumber => self.as_number() == other.as_number(),
            _ => false
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value_type {
            ValueType::ValBool => {
                if self.as_bool() {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            ValueType::ValNil => write!(f, "nil"),
            ValueType::ValNumber => write!(f, "{}", self.as_number()),
        }
    }
}

#[derive(Default)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn write_value_array(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn free_value_array(&mut self) {
        self.values.clear();
    }
}
