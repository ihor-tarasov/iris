use crate::common::*;
use crate::module::*;
use crate::state::*;
use crate::value::*;

#[derive(Debug, Clone, Copy)]
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
    StoreLocal(usize),
    LoadLocal(usize),
    Push,
    Drop,
}

#[derive(Debug)]
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
                location: self.modules[state.module_index()].functions[state.function_index()]
                    .locations[state.opcode_index() - 1]
                    .clone(),
            }),
        }
    }

    fn jump(&mut self, state: &mut State, position: usize) {
        *state.opcode_index_mut() = position;
    }

    fn jump_if(&mut self, state: &mut State, value: bool, position: usize) -> Result<(), Error> {
        let value_from_stack = state.pop();
        match value_from_stack {
            Value::Bool(value_from_stack) => {
                if value_from_stack == value {
                    self.jump(state, position);
                }
                Ok(())
            }
            _ => Err(Error {
                message: format!("Expected bool value, but got {:?}.", value_from_stack,),
                location: self.modules[state.module_index()].functions[state.function_index()]
                    .locations[state.opcode_index()]
                .clone(),
            }),
        }
    }

    fn run_module(&mut self, index: usize) -> Result<Value, Error> {
        let mut state = State::new(self.modules[index].functions[0].frame_size, index);
        while let Some(&opcode) = self.modules[state.module_index()].functions
            [state.function_index()]
        .opcodes
        .get(state.opcode_index())
        {
            *state.opcode_index_mut() += 1;
            match opcode {
                Opcode::Constant(index) => {
                    state.push(self.modules[state.module_index()].constants[index])
                }
                Opcode::Addict => self.binary::<ArithmeticOrComparison<Addict>>(&mut state)?,
                Opcode::Subtract => self.binary::<ArithmeticOrComparison<Subtract>>(&mut state)?,
                Opcode::Multiply => self.binary::<ArithmeticOrComparison<Multiply>>(&mut state)?,
                Opcode::Divide => self.binary::<ArithmeticOrComparison<Divide>>(&mut state)?,
                Opcode::Modulo => self.binary::<ArithmeticOrComparison<Modulo>>(&mut state)?,
                Opcode::And => self.binary::<Bitwise<And>>(&mut state)?,
                Opcode::Or => self.binary::<Bitwise<Or>>(&mut state)?,
                Opcode::Xor => self.binary::<Bitwise<Xor>>(&mut state)?,
                Opcode::Shl => self.binary::<Bitwise<Shl>>(&mut state)?,
                Opcode::Shr => self.binary::<Bitwise<Shr>>(&mut state)?,
                Opcode::Equal => self.binary::<Equality<Equal>>(&mut state)?,
                Opcode::NotEqual => self.binary::<Equality<NotEqual>>(&mut state)?,
                Opcode::Greater => self.binary::<ArithmeticOrComparison<Greater>>(&mut state)?,
                Opcode::Less => self.binary::<ArithmeticOrComparison<Less>>(&mut state)?,
                Opcode::GreaterEqual => {
                    self.binary::<ArithmeticOrComparison<GreaterEqual>>(&mut state)?
                }
                Opcode::LessEqual => {
                    self.binary::<ArithmeticOrComparison<LessEqual>>(&mut state)?
                }
                Opcode::JumpFalse(position) => self.jump_if(&mut state, false, position)?,
                Opcode::JumpTrue(position) => self.jump_if(&mut state, true, position)?,
                Opcode::Jump(position) => self.jump(&mut state, position),
                Opcode::StoreLocal(position) => *state.local_mut(position) = state.pop(),
                Opcode::LoadLocal(position) => state.push(*state.local(position)),
                Opcode::Push => state.push(state.peek()),
                Opcode::Drop => state.pop_drop(),
            }
        }
        Ok(state.pop())
    }

    pub fn run(&mut self) -> Result<Value, Error> {
        let mut result = Value::Bool(false);
        for i in 0..self.modules.len() {
            result = self.run_module(i)?;
        }
        Ok(result)
    }
}
