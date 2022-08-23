use crate::opcode::Opcode;

pub struct Function {
    pub opcodes: Box<[Opcode]>,
    pub locations: Box<[std::ops::Range<usize>]>,
}

pub struct FunctionBuilder {
    opcodes: Vec<Opcode>,
    locations: Vec<std::ops::Range<usize>>,
}

impl FunctionBuilder {
    pub fn new() -> Self {
        Self {
            opcodes: Vec::new(),
            locations: Vec::new(),
        }
    }

    pub fn push(&mut self, opcode: Opcode, location: std::ops::Range<usize>) {
        self.opcodes.push(opcode);
        self.locations.push(location);
    }

    pub fn len(&self) -> usize {
        self.opcodes.len()
    }

    pub fn build(self) -> Function {
        Function {
            opcodes: self.opcodes.into_boxed_slice(),
            locations: self.locations.into_boxed_slice(),
        }
    }
}