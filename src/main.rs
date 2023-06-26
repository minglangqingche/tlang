use std::env;
use tlang::{interpreter_error, chunk::{chunk, op, value::*}, debug_tools::disassemble, vm::{vm::*, interpret_result::InterpretResult}};

fn main() {
    let mut chunk = chunk::Chunk::new();

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
    
    // -((1.2+3.4)/5.6) = -0.8214285714285714
    let line = 0;
    let v = chunk.push_val(Value::Double(1.2));
    chunk.push_op(op::Opcode::OP_CONST(v), line);
    let v = chunk.push_val(Value::Double(3.4));
    chunk.push_op(op::Opcode::OP_CONST(v), line);
    chunk.push_op(op::Opcode::OP_ADD, line);
    let v = chunk.push_val(Value::Double(5.6));
    chunk.push_op(op::Opcode::OP_CONST(v), line);
    chunk.push_op(op::Opcode::OP_DIVIDE, line);
    chunk.push_op(op::Opcode::OP_NEGATE, line);

    chunk.push_op(op::Opcode::OP_RETURN, 0);
    
    // disassemble::disassemble_chunk(&chunk, "test");

    let mut vm = VM::from(chunk);
    match vm.set_debug(true).run() {
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
