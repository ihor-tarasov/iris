use std::collections::HashMap;

use crate::{function::Function, value::Value};

pub struct Module {
    pub functions: Box<[Function]>,
    pub constants: Box<[Value]>,
}

pub struct ModuleBuilder<'a> {
    pub source: &'a [u8],
    functions: Vec<Function>,
    constants: Vec<Value>,
    constants_map: HashMap<Value, usize>,
}

impl<'a> ModuleBuilder<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            functions: Vec::new(),
            constants: Vec::new(),
            constants_map: HashMap::new(),
        }
    }

    pub fn push_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    pub fn push_constant(&mut self, value: Value) -> usize {
        if let Some(&value) = self.constants_map.get(&value) {
            value
        } else {
            self.constants_map.insert(value, self.constants.len());
            self.constants.push(value);
            self.constants.len() - 1
        }
    }

    pub fn build(self) -> Module {
        Module {
            functions: self.functions.into_boxed_slice(),
            constants: self.constants.into_boxed_slice(),
        }
    }
}
