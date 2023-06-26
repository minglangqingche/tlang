use std::env;
use tlang::{interpreter_error, chunk::{chunk, op, value::*}, debug_tools::disassemble, vm::{vm::*, interpret_result::InterpretResult}};

fn main() {
    // let args: Vec<String> = env::args().collect();

    // if args.len() > 2 {
    //     interpreter_error::error_exit("Usage: jlox [script]", 64);
    // }else if args.len() < 2 {
    //     // todo run with tshell
    //     tlang::tshlle::tshlle();
    // }else {
    //     // todo run with tlang
    //     tlang::tlang::vm::run(&args[1]);
    // }

    let mut chunk = chunk::Chunk::new();
    let v = chunk.push_val(Value::Double(0.05));
    chunk.push_op(op::Opcode::Op_CONST(v), 0);
    let v = chunk.push_val(Value::Double(100.0));
    chunk.push_op(op::Opcode::Op_CONST(v), 1);
    chunk.push_op(op::Opcode::Op_RETURN, 2);
    
    // disassemble::disassemble_chunk(&chunk, "test");

    let mut vm = VM::from(chunk);
    vm.set_debug(true);
    match vm.run() {
        InterpretResult::Ok => {
            println!("program over. no error");
        },
        InterpretResult::COMPLE_ERROR => {
            interpreter_error::error_exit("have some comple error!", 64);
        },
        InterpretResult::RUNTIME_ERROR => {
            interpreter_error::error_exit("have some runtime error!", 64);
        },
    }
}
