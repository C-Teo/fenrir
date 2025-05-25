use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    
    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instructions(chunk, offset)
    }
}

fn disassemble_instructions(chunk: &Chunk, offset: usize) -> usize {
    let instruction = chunk.code[offset];
    
    match instruction {
        _ if instruction == OpCode::OP_RETURN as u8 => {
            println!("{:04x} {}", offset, "OP_RETURN");
        }
        _ => {
            println!("{:04x} {}", offset, "UNKNOWN OPCODE");
        }
    }
    offset + 1
}