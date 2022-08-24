use crate::common::*;
use crate::module::*;
use crate::opcode::Opcode;
use crate::state::*;
use crate::value::*;

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
            }
            state.opcode_index += 1;
        }
        Ok(state.stack.pop().unwrap())
    }
}
