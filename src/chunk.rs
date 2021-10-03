use crate::value::Value;

pub const OP_RETURN: usize = 0x00;
pub const OP_CONSTANT: usize = 0x01;

pub struct Chunk {
    pub code: Vec<usize>,
    pub lines: Vec<i32>,
    pub constants: Vec<Value>,
}

pub fn init_chunk() -> Chunk {
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

pub fn add_constant(chunk: &mut Chunk, value: Value) -> usize {
    chunk.constants.push(value);
    return chunk.constants.len() - 1;
}