use std::collections::HashMap;

use crate::program::Opcode;

#[derive(Debug)]
pub struct Function {
    pub opcodes: Box<[Opcode]>,
    pub locations: Box<[std::ops::Range<usize>]>,
    pub frame_size: usize,
}

pub struct FunctionBuilder {
    opcodes: Vec<Opcode>,
    locations: Vec<std::ops::Range<usize>>,
    blocks: Vec<usize>,
    frame_size: usize,
    locals: HashMap<String, usize>,
}

impl FunctionBuilder {
    pub fn new() -> Self {
        Self {
            opcodes: Vec::new(),
            locations: Vec::new(),
            blocks: vec![0],
            frame_size: 0,
            locals: HashMap::new(),
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

    pub fn enter_block(&mut self) {
        self.blocks.push(*self.blocks.last().unwrap());
    }

    pub fn exit_block(&mut self) {
        self.blocks.pop().unwrap();
    }

    pub fn get_local(&self, name: &String) -> Option<usize> {
        self.locals.get(name).cloned()
    }

    pub fn new_local(&mut self, name: &String) -> usize {
        *self.blocks.last_mut().unwrap() += 1;
        let local_position = *self.blocks.last().unwrap() - 1;
        if local_position + 1 > self.frame_size {
            self.frame_size = local_position + 1;
        }

        if let Some(index) = self.locals.get_mut(name) {
            *index = local_position;
        } else {
            self.locals.insert(name.clone(), local_position);
        }

        local_position
    }

    pub fn build(self) -> Function {
        Function {
            opcodes: self.opcodes.into_boxed_slice(),
            locations: self.locations.into_boxed_slice(),
            frame_size: self.frame_size,
        }
    }
}
