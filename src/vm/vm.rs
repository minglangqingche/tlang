use crate::{debug_tools::disassemble, chunk::{chunk::*, op::Opcode, value::*,}};
use super::interpret_result::*;

pub struct VM {
    chunk: Chunk,
    ip: Option<usize>,
    
    debug: bool,

    stack: Vec<Value>,
    stack_max: usize,
}

impl VM {
    pub fn from(chunk: Chunk) -> Self {
        Self {
            ip: if chunk.op_len() > 0 { Some(0) }else { None },
            chunk,
            debug: false,
            stack: Vec::new(),
            stack_max: 128,
        }
    }

    pub fn set_stack_max(&mut self, size: usize) {
        self.stack_max = size;
    }

    pub fn set_chunk(& mut self, c: Chunk) -> &mut Self {
        self.chunk = c;
        if self.chunk.op_len() > 0 {
            self.ip = Some(0);
        }
        self
    }

    pub fn set_debug(&mut self, f: bool) {
        self.debug = f;
    }

    pub fn run(&mut self) -> InterpretResult {
        if self.ip.eq(&None) {
            return InterpretResult::RUNTIME_ERROR;
        }
        
        loop {
            if self.debug {
                if self.ip.ne(&None) {
                    disassemble::disassemble_instruction(&self.chunk, self.ip.unwrap());
                }else{
                    return InterpretResult::RUNTIME_ERROR;
                }
            }

            let a = match self.advance() {
                None => {
                    return InterpretResult::RUNTIME_ERROR;
                },
                Some(val) => {
                    val
                }
            };

            match *a {
                Opcode::Op_CONST(val) => {
                    println!("{}", self.chunk.get_val(val).to_string());
                },
                Opcode::Op_RETURN => {
                    println!("OP_RETURN");
                    return InterpretResult::Ok;
                },
            }
        }
    }

    pub fn advance(&mut self) -> Option<&Opcode> {
        match &mut self.ip {
            Some(p) => {
                let ret = self.chunk.get_op(*p);
                *p += 1;
                ret
            },
            None => {
                None
            }
        }
    }
}
