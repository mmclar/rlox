use crate::chunk::{Chunk, init_chunk, write_chunk, add_constant, OP_CONSTANT, OP_RETURN, OP_NEGATE, OP_ADD, OP_DIVIDE};
use crate::debug::disassemble_chunk;
use crate::vm::interpret;

mod value;
mod chunk;
mod debug;
mod vm;

fn main() {
    let mut chunk: Chunk = init_chunk();

    let mut constant = add_constant(&mut chunk, 1.2);
    write_chunk(&mut chunk, OP_CONSTANT, 123);
    write_chunk(&mut chunk, constant, 123);

    constant = add_constant(&mut chunk, 3.4);
    write_chunk(&mut chunk, OP_CONSTANT, 123);
    write_chunk(&mut chunk, constant, 123);

    write_chunk(&mut chunk, OP_ADD, 123);

    constant = add_constant(&mut chunk, 5.6);
    write_chunk(&mut chunk, OP_CONSTANT, 123);
    write_chunk(&mut chunk, constant, 123);


    write_chunk(&mut chunk, OP_DIVIDE, 123);
    write_chunk(&mut chunk, OP_NEGATE, 123);

    write_chunk(&mut chunk, OP_RETURN, 123);
    disassemble_chunk(&chunk, String::from("test chunk"));
    interpret(chunk);
}
