use std::mem::ManuallyDrop;
use std::{any::Any, fmt, rc::Rc};

use crate::common::ValueType;
use crate::object::{Obj, ObjString, ObjType};

#[repr(C)]
pub union InnerValue {
    boolean: bool,
    number: i64,
    obj: ManuallyDrop<Rc<Obj>>,
}

impl Drop for InnerValue {
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.obj) }
    }
}
pub struct Value {
    pub value_type: ValueType,
    pub value: InnerValue,
}

impl Clone for Value {
    fn clone(&self) -> Self {
        unsafe {
            match self.value_type {
                ValueType::ValBool => {
                    return Self {
                        value_type: self.value_type,
                        value: InnerValue {
                            boolean: self.value.boolean,
                        },
                    }
                }
                ValueType::ValNil => {
                    return Self {
                        value_type: self.value_type,
                        value: InnerValue {
                            number: self.value.number,
                        },
                    }
                }
                ValueType::ValNumber => {
                    return Self {
                        value_type: self.value_type,
                        value: InnerValue {
                            number: self.value.number,
                        },
                    }
                }
                ValueType::ValObj => {
                    return Self {
                        value_type: self.value_type,
                        value: InnerValue {
                            obj: ManuallyDrop::new(Rc::clone(&(*self.value.obj))),
                        },
                    }
                }
            }
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
            value: InnerValue { boolean: val },
        }
    }

    pub fn new_nil() -> Self {
        Value {
            value_type: ValueType::ValNil,
            value: InnerValue { number: 0 },
        }
    }

    pub fn new_number(val: i64) -> Self {
        Value {
            value_type: ValueType::ValNumber,
            value: InnerValue { number: val },
        }
    }

    pub fn new_obj(obj: Obj) -> Self {
        Value {
            value_type: ValueType::ValObj,
            value: InnerValue {
                obj: ManuallyDrop::new(Rc::new(obj)),
            },
        }
    }

    pub fn as_bool(&self) -> bool {
        unsafe {
            return self.value.boolean;
        }
    }

    pub fn as_number(&self) -> i64 {
        unsafe {
            return self.value.number;
        }
    }

    pub fn as_obj(&self) -> Rc<Obj> {
        unsafe {
            return Rc::clone(&(*self.value.obj));
        }
    }

    pub fn as_string(&self) -> Rc<ObjString> {
        unsafe { std::mem::transmute::<Rc<Obj>, Rc<ObjString>>(self.as_obj()) }
    }

    pub fn as_rust_string(&self) -> String {
        self.as_string().string.clone()
    }

    pub fn is_bool(&self) -> bool {
        self.value_type == ValueType::ValBool
    }

    pub fn is_nil(&self) -> bool {
        self.value_type == ValueType::ValNil
    }

    pub fn is_number(&self) -> bool {
        self.value_type == ValueType::ValNumber
    }

    pub fn is_obj(&self) -> bool {
        self.value_type == ValueType::ValObj
    }

    pub fn is_falsey(&self) -> bool {
        self.is_nil() || (self.is_bool() && !self.as_bool())
    }

    pub fn obj_type(&self) -> ObjType {
        self.as_obj().obj_type
    }

    pub fn is_obj_type(&self, obj_type: ObjType) -> bool {
        return self.is_obj() && self.obj_type() == obj_type;
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
            ValueType::ValObj => todo!(),
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
            _ => false,
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
            ValueType::ValObj => todo!(),
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
