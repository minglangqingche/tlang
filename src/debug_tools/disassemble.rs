use crate::chunk::{chunk::*, op::*,};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("====$ {} $====", name);

    let mut offset = 0;
    while  offset < chunk.op_len() {
        disassemble_instruction(chunk, offset);
        offset += 1;
    }
    println!("====$ over $====");
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    let instruction = chunk.get_op(offset).unwrap();
    match instruction {
        Opcode::Op_RETURN => {
            println!("{} {:04}$ OP_RETURN,", get_line(chunk, offset), offset);
        },
        Opcode::Op_CONST(index) => {
            let val = chunk.get_val(*index).to_string();
            println!("{} {:04}$ OP_CONST {},", get_line(chunk, offset), offset, val);
        },
    }
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
