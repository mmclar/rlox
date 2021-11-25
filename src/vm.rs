use crate::chunk::{Chunk, OP_RETURN, OP_CONSTANT, OP_NEGATE, OP_ADD, OP_SUBTRACT, OP_MULTIPLY, OP_DIVIDE, init_chunk};
use crate::compiler::Compiler;
use crate::value::{as_number, number_val, print_value, Value, ValueData, ValueType};
use crate::debug::disassemble_instruction;

pub const DEBUG_TRACE_EXECUTION: bool = true;

pub(crate) type InterpretResult = usize;
pub const INTERPRET_OK: InterpretResult = 0;
pub const INTERPRET_COMPILE_ERROR: InterpretResult = 1;
pub const INTERPRET_RUNTIME_ERROR: InterpretResult = 2;

pub struct VM {
    chunk: Chunk,
    stack: Vec<Value>,
}

pub fn interpret(source: String) -> InterpretResult {
    let chunk = init_chunk();
    let mut compiler = Compiler::new(source, chunk);
    if !compiler.compile() { return INTERPRET_COMPILE_ERROR; }
    let vm = VM {
        chunk: compiler.compiling_chunk,
        stack: Vec::new(),
    };
    return run(vm)
}

// , l: &dyn Fn(Value, Value) -> Value
fn bin_op(mut stack: Vec<Value>, op_fn: &dyn Fn(f64, f64) -> f64) -> (Vec<Value>, InterpretResult) {
    match stack.pop() {
        None => return (stack, INTERPRET_RUNTIME_ERROR),
        Some(b) => {
            match stack.pop() {
                None => return (stack, INTERPRET_RUNTIME_ERROR),
                Some(a) => {
                    stack.push(number_val(op_fn(as_number(&a), as_number(&b))));
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
        let none = |_a: f64, _b: f64| 0.0;
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
                Some(value) => stack.push(number_val(-as_number(&value))),
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