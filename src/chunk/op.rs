#[allow(non_camel_case_types)]
pub enum Opcode {
    OP_RETURN,
    OP_CONST(usize), // val_index
    OP_NEGATE,
    OP_ADD,
    OP_SUB,
    OP_MULTIPLY,
    OP_DIVIDE,
}

impl Copy for Opcode {}

impl Clone for Opcode {
    fn clone(&self) -> Self {
        *self
    }
}
