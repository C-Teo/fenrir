mod chunk;
mod debug;
mod value;

use chunk::Chunk;
use debug::disassemble_chunk;
use crate::chunk::OpCode::{OpConstant, OpReturn};

fn main() { 
    // Init Chunk
    let mut chunk = Chunk::new();
    
    // Write to Chunk
    let constant: usize = chunk.add_constant(1.2);
    chunk.write_chunk(OpConstant as u8);
    chunk.write_chunk(constant as u8);
    chunk.write_chunk(OpReturn as u8);
    
    // Chunk Deallocate
    disassemble_chunk(&chunk, "Chunk")
}


