pub type Value = f64;

pub fn print_value(value: Value) {
    print!("{}", value);
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
