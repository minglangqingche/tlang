#[allow(non_camel_case_types)]
pub enum Opcode {
    Op_RETURN,
    Op_CONST(usize), // val_index
}

impl Copy for Opcode {}

impl Clone for Opcode {
    fn clone(&self) -> Self {
        *self
    }
}
