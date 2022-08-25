use crate::program::Opcode;

#[derive(Debug)]
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

    pub fn push_unknown(&mut self, location: std::ops::Range<usize>) -> usize {
        self.opcodes.push(Opcode::Addict);
        self.locations.push(location);
        self.opcodes.len() - 1
    }

    pub fn set(&mut self, position: usize, opcode: Opcode) {
        self.opcodes[position] = opcode;
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
