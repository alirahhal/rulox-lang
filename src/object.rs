#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ObjType {
    ObjString,
}

pub trait Obj {
    fn obj_type(&self) -> ObjType;
    fn as_obj_string(&self) -> Option<&ObjString>;
}

pub struct ObjString {
    pub obj_type: ObjType,
    pub string: String,
}

impl ObjString {
    pub fn new(str: String) -> Self {
        ObjString {
            obj_type: ObjType::ObjString,
            string: str,
        }
    }
}

impl Obj for ObjString {
    fn obj_type(&self) -> ObjType {
        self.obj_type
    }

    fn as_obj_string(&self) -> Option<&ObjString> {
        match self.obj_type {
            ObjType::ObjString => Some(self),
        }
    }
}

impl Clone for ObjString {
    fn clone(&self) -> Self {
        Self {
            obj_type: self.obj_type,
            string: self.string.to_owned(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}
