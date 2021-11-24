use crate::value::Value;

pub const OP_RETURN: usize = 1;
pub const OP_CONSTANT: usize = 0;
pub const OP_NEGATE: usize = 2;
pub const OP_ADD: usize = 3;
pub const OP_SUBTRACT: usize = 4;
pub const OP_MULTIPLY: usize = 5;
pub const OP_DIVIDE: usize = 6;

pub struct Chunk {
    pub code: Vec<usize>,
    pub lines: Vec<i32>,
    pub constants: Vec<Value>,
}

pub const fn init_chunk() -> Chunk {
    return Chunk {
        code: Vec::new(),
        lines: Vec::new(),
        constants: Vec::new(),
    }
}

pub fn write_chunk(chunk: &mut Chunk, byte: usize, line: i32) {
    chunk.code.push(byte);
    chunk.lines.push(line);
}

// pub fn add_constant(chunk: &mut Chunk, value: Value) -> usize {
//     chunk.constants.push(value);
//     return chunk.constants.len() - 1;
// }