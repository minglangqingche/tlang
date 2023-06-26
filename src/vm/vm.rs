use crate::{debug_tools::disassemble, chunk::{chunk::*, op::Opcode,}};
use super::interpret_result::*;

pub struct VM {
    chunk: Chunk,
    ip: Option<usize>,
    debug: bool,
}

impl VM {
    pub fn from(chunk: Chunk) -> Self {
        Self {
            ip: if chunk.op_len() > 0 { Some(0) }else { None },
            chunk,
            debug: false,
        }
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
                    let v = match self.chunk.get_val(val) {
                        Some(a) => {
                            a.to_string()
                        },
                        None => {
                            "null".to_string()
                        }
                    };
                    println!("{}", v);
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
