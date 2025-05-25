pub type Value = f64;

#[derive(Debug)]
pub struct ValueArray {
    values: Vec<Value>
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray {
            values: Vec::new()
        }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }
    
    pub fn get(&self, index: usize) -> Value {
        self.values[index]
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

}