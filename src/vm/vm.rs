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

    pub fn set_debug(&mut self, f: bool) -> &mut Self {
        self.debug = f;
        self
    }

    pub fn run(&mut self) -> InterpretResult {
        if self.ip.eq(&None) {
            return InterpretResult::RUNTIME_ERROR;
        }
        
        loop {
            if self.debug {
                if self.ip.ne(&None) {
                    println!("stack:{:?}", self.stack);
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
                Opcode::OP_CONST(val) => {
                    if self.stack.len() == self.stack_max {
                        return InterpretResult::RUNTIME_ERROR;
                    }

                    self.stack.push(self.chunk.get_val(val).clone());
                },
                Opcode::OP_RETURN => {
                    if self.stack.is_empty() {
                        println!("stack is empty!");
                    }else {
                        println!("stack top = {}", self.stack.pop().unwrap().to_string());
                    }

                    return InterpretResult::Ok;
                },
                Opcode::OP_NEGATE => {
                    if self.stack.is_empty() {
                        return InterpretResult::RUNTIME_ERROR;
                    }

                    let a = self.stack.last_mut().unwrap();
                    match a {
                        Value::Double(d) => *a = Value::Double(-(*d)),
                        _ => return InterpretResult::RUNTIME_ERROR,
                    }
                },
                Opcode::OP_ADD => {
                    if self.stack.len() < 2 {
                        return InterpretResult::RUNTIME_ERROR;
                    }

                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left.add(&right));
                },
                Opcode::OP_SUB => {
                    if self.stack.len() < 2 {
                        return InterpretResult::RUNTIME_ERROR;
                    }

                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left.sub(&right));
                },
                Opcode::OP_MULTIPLY => {
                    if self.stack.len() < 2 {
                        return InterpretResult::RUNTIME_ERROR;
                    }

                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left.multiply(&right));
                },
                Opcode::OP_DIVIDE => {
                    if self.stack.len() < 2 {
                        return InterpretResult::RUNTIME_ERROR;
                    }

                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left.divide(&right));
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
