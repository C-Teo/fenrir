mod chunk;
mod debug;

use chunk::Chunk;
use debug::disassemble_chunk;
use crate::chunk::OpCode::OP_RETURN;

fn main() { 
    // Init Chunk
    let mut chunk = Chunk::new();
    
    // Write to Chunk
    chunk.write_chunk(OP_RETURN as u8);
    
    // Chunk Deallocate
    disassemble_chunk(&chunk, "test chunk")
}


