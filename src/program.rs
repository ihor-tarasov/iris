use crate::common::*;
use crate::module::*;
use crate::state::*;
use crate::value::*;

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
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    JumpFalse(usize),
    JumpTrue(usize),
    Jump(usize),
}

pub struct Program {
    modules: Vec<Module>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    pub fn push(&mut self, module: Module) {
        self.modules.push(module);
    }

    fn binary<T: BinaryOperator>(&mut self, state: &mut State) -> Result<(), Error> {
        match state.binary::<T>() {
            Ok(_) => Ok(()),
            Err(message) => Err(Error {
                message,
                location: self.modules[state.module_index].functions[state.function_index]
                    .locations[state.opcode_index - 1]
                    .clone(),
            }),
        }
    }

    fn jump(&mut self, state: &mut State, position: usize) {
        state.opcode_index = position;
    }

    fn jump_if(&mut self, state: &mut State, value: bool, position: usize) -> Result<(), Error> {
        let value_from_stack = state.stack.pop().unwrap();
        match value_from_stack {
            Value::Bool(value_from_stack) => {
                if value_from_stack == value {
                    self.jump(state, position);
                }
                Ok(())
            }
            _ => Err(Error {
                message: format!(
                    "Expected bool value, but got {}({:?}).",
                    value_from_stack.type_name(),
                    value_from_stack
                ),
                location: self.modules[state.module_index].functions[state.function_index]
                    .locations[state.opcode_index]
                    .clone(),
            }),
        }
    }

    pub fn run(&mut self, state: &mut State) -> Result<Value, Error> {
        while let Some(&opcode) = self.modules[state.module_index].functions[state.function_index]
            .opcodes
            .get(state.opcode_index)
        {
            state.opcode_index += 1;
            match opcode {
                Opcode::Constant(index) => state
                    .stack
                    .push(self.modules[state.module_index].constants[index]),
                Opcode::Addict => self.binary::<ArithmeticOrComparison<Addict>>(state)?,
                Opcode::Subtract => self.binary::<ArithmeticOrComparison<Subtract>>(state)?,
                Opcode::Multiply => self.binary::<ArithmeticOrComparison<Multiply>>(state)?,
                Opcode::Divide => self.binary::<ArithmeticOrComparison<Divide>>(state)?,
                Opcode::Modulo => self.binary::<ArithmeticOrComparison<Modulo>>(state)?,
                Opcode::And => self.binary::<Bitwise<And>>(state)?,
                Opcode::Or => self.binary::<Bitwise<Or>>(state)?,
                Opcode::Xor => self.binary::<Bitwise<Xor>>(state)?,
                Opcode::Shl => self.binary::<Bitwise<Shl>>(state)?,
                Opcode::Shr => self.binary::<Bitwise<Shr>>(state)?,
                Opcode::Equal => self.binary::<Equality<Equal>>(state)?,
                Opcode::NotEqual => self.binary::<Equality<NotEqual>>(state)?,
                Opcode::Greater => self.binary::<ArithmeticOrComparison<Greater>>(state)?,
                Opcode::Less => self.binary::<ArithmeticOrComparison<Less>>(state)?,
                Opcode::GreaterEqual => {
                    self.binary::<ArithmeticOrComparison<GreaterEqual>>(state)?
                }
                Opcode::LessEqual => self.binary::<ArithmeticOrComparison<LessEqual>>(state)?,
                Opcode::JumpFalse(position) => self.jump_if(state, false, position)?,
                Opcode::JumpTrue(position) => self.jump_if(state, true, position)?,
                Opcode::Jump(position) => self.jump(state, position),
            }
        }
        Ok(state.stack.pop().unwrap())
    }
}
