use crate::value::*;

pub struct State {
    pub stack: Vec<Value>,
    pub module_index: usize,
    pub function_index: usize,
    pub opcode_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            module_index: 0,
            function_index: 0,
            opcode_index: 0,
        }
    }

    pub fn binary<T: BinaryOperator>(&mut self) -> Result<(), String> {
        let rhs = self.stack.pop().unwrap();
        let lhs = self.stack.pop().unwrap();
        self.stack.push(T::eval(lhs, rhs)?);
        Ok(())
    }
}
