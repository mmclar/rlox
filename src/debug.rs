use crate::chunk::{Chunk, OP_RETURN, OP_CONSTANT, OP_NEGATE, OP_ADD, OP_SUBTRACT, OP_MULTIPLY, OP_DIVIDE};
use crate::value::print_value;
// use crate::value::print_value;

// pub fn disassemble_chunk(chunk: &Chunk, name: String) {
//     println!("== {} == ", name);
//
//     let mut offset = 0;
//     while offset < chunk.code.len() {
//         offset = disassemble_instruction(chunk, offset);
//     }
// }

fn constant_instruction(name: String, chunk: &Chunk, offset: usize) -> usize {
    let constant_idx = chunk.code[offset + 1];
    print!("{:16} {:4} '", name, constant_idx);
    print_value(chunk.constants[constant_idx]);
    print!("'\n");
    return offset + 2;
}

fn simple_instruction(name: String, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
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
    else if instruction == OP_ADD {
        return simple_instruction(String::from("OP_ADD"), offset);
    }
    else if instruction == OP_SUBTRACT {
        return simple_instruction(String::from("OP_SUBTRACT"), offset);
    }
    else if instruction == OP_MULTIPLY {
        return simple_instruction(String::from("OP_MULTIPLY"), offset);
    }
    else if instruction == OP_DIVIDE {
        return simple_instruction(String::from("OP_DIVIDE"), offset);
    }
    else if instruction == OP_NEGATE {
        return simple_instruction("OP_NEGATE".to_string(), offset);
    }
    println!("Unknown opcode {:?}", instruction);
    return offset + 1;
}