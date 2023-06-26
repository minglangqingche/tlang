use crate::chunk::{chunk::*, op::*,};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("====$ {} $====", name);

    for offset in 0..chunk.op_len() {
        disassemble_instruction(chunk, offset);
    }

    println!("====$ over $====");
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    let print_code = | massege | {
        println!("{} {:04}$ {},",
            match chunk.get_line(offset) {
                Some(line) => {
                    if offset > 0 && chunk.get_line(offset - 1).unwrap() == line {
                        format!("   |")
                    }else {
                        format!("{:04}", line)
                    }
                },
                None => format!("   ?"),
            },
            offset, massege);
    };

    let instruction = chunk.get_op(offset).unwrap();
    match instruction {
        Opcode::OP_RETURN => {
            print_code("OP_RETURN");
        },
        Opcode::OP_CONST(index) => {
            let val = chunk.get_val(*index).to_string();
            print_code(&format!("OP_CONST {}", val));
        },
        Opcode::OP_NEGATE => {
            print_code("OP_NEGATE");
        },
        Opcode::OP_ADD => {
            print_code("OP_ADD");
        },
        Opcode::OP_DIVIDE => {
            print_code("OP_DIVIDE");
        },
        Opcode::OP_MULTIPLY => {
            print_code("OP_MULTIPLY");
        },
        Opcode::OP_SUB => {
            print_code("OP_SUBTRACT");
        }
    }
}
