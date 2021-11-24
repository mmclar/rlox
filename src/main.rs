use std::{env, io};
use std::io::Write;
use std::fs;

// use crate::chunk::{OP_DIVIDE, OP_NEGATE, OP_RETURN, write_chunk};
// use crate::debug::disassemble_chunk;
use crate::vm::interpret;
use crate::vm::InterpretResult;

mod value;
mod chunk;
mod debug;
mod vm;
mod compiler;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len == 1 {
        repl();
    }
    if arg_len == 2 {
        run_file(&args[1]);
    }
    else {
        panic!("Usage: rlox [path]\n");
    }
}

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => { print!("{}", line); }
            Err(_) => {},
        }
    }
}

fn run_file(path: &String) {
    let source = fs::read_to_string(path).expect("Something went wrong reading the file");
    let _result: InterpretResult  = interpret(source);

    // if (result == INTERPRET_COMPILE_ERROR) exit(65);
    // if (result == INTERPRET_RUNTIME_ERROR) exit(70);
}
