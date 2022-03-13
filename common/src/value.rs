use std::fmt::Debug;
use std::rc::Rc;

use crate::object::Obj;

#[derive(Clone)]
pub enum Value {
    Boolean(bool),
    Number(i64),
    Object(Rc<Obj>),
    Nil,
}

impl Debug for Value {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(v) => {
                if *v {
                    print!("true");
                } else {
                    print!("false");
                }
            }
            Value::Number(v) => print!("{}", *v),
            Value::Object(r) => match &**r {
                Obj::String(v) => print!("{}", v),
            },
            Value::Nil => print!("nil"),
        };
        Ok(())
    }
}

impl Value {
    pub fn new_bool(val: bool) -> Self {
        Value::Boolean(val)
    }

    pub fn new_nil() -> Self {
        Value::Nil
    }

    pub fn new_number(val: i64) -> Self {
        Value::Number(val)
    }

    pub fn new_obj(obj: Rc<Obj>) -> Self {
        Value::Object(obj)
    }

    pub fn new_obj_string(s: String) -> Self {
        Value::Object(Rc::new(Obj::String(s)))
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Boolean(v) => *v,
            _ => panic!(),
        }
    }

    pub fn as_number(&self) -> i64 {
        match self {
            Value::Number(v) => *v,
            _ => panic!(),
        }
    }

    pub fn as_obj(&self) -> &Rc<Obj> {
        match self {
            Value::Object(r) => r,
            _ => panic!(),
        }
    }

    pub fn as_string(&self) -> &str {
        match &**self.as_obj() {
            Obj::String(v) => v,
            // _ => panic!(),
        }
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_obj(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Boolean(v) => !*v,
            _ => true,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::Object(v) => matches!(**v, Obj::String(_)),
            _ => false,
        }
    }

    pub fn print_value(&self) {
        print!("{:?}", self)
    }

    pub fn values_equal(&self, other: &Self) -> bool {
        if core::mem::discriminant(self) != core::mem::discriminant(other) {
            return false;
        }

        match self {
            Value::Boolean(_) => self.as_bool() == other.as_bool(),
            Value::Nil => true,
            Value::Number(_) => self.as_number() == other.as_number(),
            Value::Object(_) => {
                let obj1 = self.as_obj();
                let obj2 = other.as_obj();

                let tuple = (&**obj1, &**obj2);

                match tuple {
                    (Obj::String(v1), Obj::String(v2)) => v1 == v2,
                    // _ => ptr::eq(obj1.as_ref(), obj2.as_ref()),
                }
            }
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
