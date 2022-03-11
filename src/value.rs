use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;

use crate::common::ValueType;
use crate::object::{Obj, ObjString, ObjType};

// #[repr(C)]
// pub union InnerValue {
//     boolean: bool,
//     number: i64,
//     obj: ManuallyDrop<Rc<dyn Obj>>,
// }

pub enum Value {
    Boolean(bool),
    Number(i64),
    // Integer(i32),
    // Double(f32),
    Object(Rc<dyn Obj>),
    Nil,
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

    pub fn new_obj(obj: Rc<dyn Obj>) -> Self {
        Value::Object(obj)
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

    pub fn as_obj(&self) -> &Rc<dyn Obj> {
        match self {
            Value::Object(r) => r,
            _ => panic!(),
        }
    }

    pub fn as_string(&self) -> &ObjString {
        self.as_obj().deref().as_obj_string()
    }

    pub fn as_rust_string(&self) -> &str {
        &self.as_string().string
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_obj(&self) -> bool {
        match self {
            Value::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Boolean(v) => *v,
            _ => true,
        }
    }

    pub fn is_string(&self) -> bool {
        self.is_obj_type(ObjType::ObjString)
    }

    pub fn obj_type(&self) -> ObjType {
        self.as_obj().obj_type()
    }

    pub fn is_obj_type(&self, obj_type: ObjType) -> bool {
        self.is_obj() && self.obj_type() == obj_type
    }

    pub fn print_value(&self) {
        match self {
            Value::Boolean(v) => {
                if *v {
                    print!("true");
                } else {
                    print!("false");
                }
            }
            Value::Number(v) => print!("{}", *v),
            Value::Object(_) => self.print_object(),
            Value::Nil => print!("nil"),
        }
    }

    pub fn print_object(&self) {
        match self.obj_type() {
            ObjType::ObjString => print!("{}", self.as_rust_string()),
        }
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
                let obj_1 = self.as_obj();
                let obj_2 = other.as_obj();

                if obj_1.obj_type() == ObjType::ObjString && obj_2.obj_type() == ObjType::ObjString {
                    self.as_rust_string() == other.as_rust_string()
                } else {
                    ptr::eq(obj_1.as_ref(), obj_2.as_ref())
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
