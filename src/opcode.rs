#[derive(Clone, Copy)]
pub enum Opcode {
    Constant(usize),
    Addict,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Xor,
    Shl,
    Shr,
}
