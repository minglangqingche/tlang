use crate::chunk::{chunk::*, op::*,};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("====$ {} $====", name);

    for offset in 0..chunk.op_len() {
        disassemble_instruction(chunk, offset);
    }

    println!("====$ over $====");
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    let instruction = chunk.get_op(offset).unwrap();
    match instruction {
        Opcode::OP_RETURN => {
            print_code(chunk, offset, "OP_RETURN");
        },
        Opcode::OP_CONST(index) => {
            let val = chunk.get_val(*index).to_string();
            print_code(chunk, offset, &format!("OP_CONST {}", val));
        },
        Opcode::OP_NEGATE => {
            print_code(chunk, offset, "OP_NEGATE");
        },
        Opcode::OP_ADD => {
            print_code(chunk, offset, "OP_ADD");
        },
        Opcode::OP_DIVIDE => {
            print_code(chunk, offset, "OP_DIVIDE");
        },
        Opcode::OP_MULTIPLY => {
            print_code(chunk, offset, "OP_MULTIPLY");
        },
        Opcode::OP_SUB => {
            print_code(chunk, offset, "OP_SUBTRACT");
        }
    }
}

fn print_code(chunk: &Chunk, offset: usize, name: &str) {
    println!("{} {:04}$ {},", get_line(chunk, offset), offset, name);
}

fn get_line(chunk: &Chunk, offset: usize) -> String {
    match chunk.get_line(offset) {
        Some(line) => {
            if offset > 0 && chunk.get_line(offset - 1).unwrap() == line {
                format!("   |")
            }else {
                format!("{:04}", line)
            }
        },
        None => format!("   ?"),
    }
}
