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
        _ if instruction == OpCode::OpReturn as u8 => {
            simple_instruction("OP_RETURN", chunk, offset)
        },
        _ if instruction == OpCode::OpConstant as u8 => {
            constant_instruction("OP_CONSTANT", chunk, offset)
        }
        _ => {
            println!("{:04x} {}", offset, "UNKNOWN OPCODE");
            offset + 1
        }
    }
}

fn simple_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let line: u32 = get_line(chunk, offset as u32);
    
    if offset > 0 && line == get_line(chunk, (offset - 1) as u32) {
        println!("{:04x} | {}", offset, name);
    } else {
        println!("{:04x} {} {}", offset, line, name);
    }
   
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant_index = chunk.code[offset + 1] as usize;
    let constant = chunk.constants.get(constant_index);
    let line: u32 = get_line(chunk, offset as u32);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        println!("{:04x} | {} {}", offset, line, constant);
    } else {
        println!("{:04x} {} {} {}", offset, line, name, constant);
    } 
    
    offset + 2
}

fn get_line(chunk: &Chunk, mut offset: u32) -> u32 { 
    for (line, occurences) in &chunk.lines {
        if *occurences < offset + 1 {
            offset -= *occurences
        } else {
            return *line;
        }
    }
    0
}