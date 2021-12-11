#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ObjType {
    ObjString,
}

pub struct Obj {
    pub obj_type: ObjType,
}

pub struct ObjString {
    pub obj: Obj,
    pub string: String,
}

impl std::ops::Deref for ObjString {
    type Target = Obj;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
