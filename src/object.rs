#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ObjType {
    ObjString,
}

#[derive(Debug, Copy, Clone)]
pub struct Obj {
    pub obj_type: ObjType,
}

pub struct ObjString {
    pub obj: Obj,
    pub string: String,
}

impl Clone for ObjString {
    fn clone(&self) -> Self {
        Self {
            obj: self.obj,
            string: self.string.to_owned(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}

impl std::ops::Deref for ObjString {
    type Target = Obj;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
