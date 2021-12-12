use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;

use crate::common::ValueType;
use crate::object::{Obj, ObjString, ObjType};

#[repr(C)]
pub union InnerValue {
    boolean: bool,
    number: i64,
    obj: ManuallyDrop<Rc<dyn Obj>>,
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
                            obj: ManuallyDrop::new(Rc::clone(&self.value.obj.deref())),
                        },
                    };
                }
            }
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        if self.is_obj() {
            unsafe { ManuallyDrop::drop(&mut self.value.obj) }
        }
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

    pub fn new_obj(obj: Rc<dyn Obj>) -> Self {
        Value {
            value_type: ValueType::ValObj,
            value: InnerValue {
                obj: ManuallyDrop::new(obj),
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

    pub fn as_obj(&self) -> Rc<dyn Obj> {
        unsafe { Rc::clone(&self.value.obj.deref()) }
    }

    pub fn as_string(&self) -> ObjString {
        let c = self.as_obj();
        c.as_obj_string().unwrap().clone()
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

    pub fn is_string(&self) -> bool {
        self.is_obj_type(ObjType::ObjString)
    }

    pub fn obj_type(&self) -> ObjType {
        self.as_obj().obj_type()
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
            ValueType::ValObj => self.print_object(),
        }
    }

    pub fn print_object(&self) {
        match self.obj_type() {
            ObjType::ObjString => print!("{}", self.as_rust_string()),
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
            ValueType::ValObj => {
                let a_obj = self.as_obj();
                let b_obj = other.as_obj();

                if a_obj.obj_type() == ObjType::ObjString {
                    return self.as_rust_string() == other.as_rust_string();
                } else {
                    return ptr::eq(a_obj.as_ref(), b_obj.as_ref());
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
    pub fn write_value_array(&mut self, value: &Value) {
        self.values.push(value.clone());
    }

    pub fn free_value_array(&mut self) {
        self.values.clear();
    }
}
