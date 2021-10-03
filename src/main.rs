use crate::chunk::{Chunk, init_chunk, write_chunk, add_constant, OP_CONSTANT, OP_RETURN};
use crate::debug::disassemble_chunk;

mod value;
mod chunk;
mod debug;

fn main() {
    let mut chunk: Chunk = init_chunk();

    let constant = add_constant(&mut chunk, 1.2);
    write_chunk(&mut chunk, OP_CONSTANT, 123);
    write_chunk(&mut chunk, constant, 123);

    write_chunk(&mut chunk, OP_RETURN, 123);

    disassemble_chunk(&chunk, "test chunk".to_string());
}
