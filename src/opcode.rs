#[derive(Clone, Copy)]
pub enum Opcode {
    Constant(usize),
    Addict,
    Multiply,
}
