use crate::value::{Value, ValueArray};

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: ValueArray::new(),
        }
    }
    
    pub fn write_chunk(&mut self, byte: u8) {
        self.code.push(byte);
    }
    
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OpConstant = 0x01,
    OpReturn = 0x02,
}
