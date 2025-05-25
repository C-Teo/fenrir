#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
        }
    }
    
    pub fn write_chunk(&mut self, byte: u8) {
        self.code.push(byte);
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OP_RETURN
}
