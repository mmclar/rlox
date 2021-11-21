use crate::chunk::{Chunk, OP_RETURN, OP_CONSTANT, OP_NEGATE, OP_ADD, OP_SUBTRACT, OP_MULTIPLY};
use crate::value::{print_value, Value};
use crate::debug::disassemble_instruction;
use crate::OP_DIVIDE;

pub const DEBUG_TRACE_EXECUTION: bool = true;

type InterpretResult = usize;
pub const INTERPRET_OK: InterpretResult = 0;
// pub const INTERPRET_COMPILE_ERROR: InterpretResult = 1;
pub const INTERPRET_RUNTIME_ERROR: InterpretResult = 2;

pub struct VM {
    chunk: Chunk,
    stack: Vec<Value>,
}

pub fn interpret(chunk: Chunk) -> InterpretResult {
    if DEBUG_TRACE_EXECUTION {
        println!("\n== Interpreting ==");
    }
    let vm = VM {
        chunk: chunk,
        stack: Vec::new(),
    };
    return run(vm);
}
// , l: &dyn Fn(Value, Value) -> Value
fn bin_op(mut stack: Vec<Value>, op_fn: &dyn Fn(Value, Value) -> Value) -> (Vec<Value>, InterpretResult) {
    match stack.pop() {
        None => return (stack, INTERPRET_RUNTIME_ERROR),
        Some(b) => {
            match stack.pop() {
                None => return (stack, INTERPRET_RUNTIME_ERROR),
                Some(a) => {
                    stack.push(op_fn(a, b));
                }
            }
        }
    }
    (stack, INTERPRET_OK)
}

fn run(vm: VM) -> InterpretResult {
    let mut inst_idx = 0;
    let mut stack = vm.stack;
    while inst_idx < vm.chunk.code.len() {

        if DEBUG_TRACE_EXECUTION {
            println!("        {:?}", stack);
            disassemble_instruction(&vm.chunk, inst_idx);
        }

        let instruction = vm.chunk.code[inst_idx];

        if instruction == OP_CONSTANT {
            inst_idx += 1;
            let const_idx = vm.chunk.code[inst_idx];
            let constant = vm.chunk.constants[const_idx];
            stack.push(constant);
        }

        // TODO: Not idiomatic way of skipping if no match
        let none = |a: Value, b: Value| 0.0;
        let op_fn = match instruction {
            OP_ADD => |a, b| a + b,
            OP_SUBTRACT => |a, b| a - b,
            OP_MULTIPLY => |a, b| a * b,
            OP_DIVIDE  => |a, b| a / b,
            _ => none,
        };
        if op_fn != none {
            let r = bin_op(stack, &op_fn);
            stack = r.0;
            if r.1 == INTERPRET_RUNTIME_ERROR {
                return r.1;
            }
        }

        else if instruction == OP_NEGATE {
            match stack.pop() {
                None => return INTERPRET_RUNTIME_ERROR,
                Some(value) => stack.push(-value),
            }
        }

        else if instruction == OP_RETURN {
            match stack.pop() {
                None => return INTERPRET_RUNTIME_ERROR,
                Some(value) => {
                    print_value(value);
                    print!("\n");
                    return INTERPRET_OK;
                }
            }
        }

        inst_idx += 1;
    }
    return INTERPRET_RUNTIME_ERROR;
}