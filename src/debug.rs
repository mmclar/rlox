use crate::chunk::{Chunk, OP_RETURN, OP_CONSTANT};
use crate::value::print_value;

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("== {} == ", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

fn constant_instruction(name: String, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{:16} {:4} '", name, constant);
    print_value(chunk.constants[constant]);
    print!("'\n");
    return offset + 2;
}

fn simple_instruction(name: String, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    }
    else {
        print!("{:4} ", chunk.lines[offset]);
    }

    let instruction = chunk.code[offset];
    if instruction == OP_RETURN {
        return simple_instruction("OP_RETURN".to_string(), offset);
    }
    else if instruction == OP_CONSTANT {
        return constant_instruction("OP_CONSTANT".to_string(), chunk, offset);
    }
    println!("Unknown opcode {:?}", instruction);
    return offset + 1;
}