use crate::value::*;

pub struct Frame(Box<[Value]>);

impl Frame {
    pub fn new(size: usize) -> Self {
        Self(vec![Value::Bool(false); size].into_boxed_slice())
    }
}

pub struct State {
    stack: Vec<Value>,
    frames: Vec<Frame>,
    module_indexes: Vec<usize>,
    function_indexes: Vec<usize>,
    opcode_indexes: Vec<usize>,
    params_counts: Vec<usize>,
}

impl State {
    pub fn new(frame_size: usize, module_index: usize) -> Self {
        Self {
            stack: Vec::new(),
            frames: vec![Frame::new(frame_size)],
            module_indexes: vec![module_index],
            function_indexes: vec![0],
            opcode_indexes: vec![0],
            params_counts: vec![0],
        }
    }

    pub fn local(&self, index: usize) -> &Value {
        self.frames.last().unwrap().0.get(index).unwrap()
    }

    pub fn local_mut(&mut self, index: usize) -> &mut Value {
        self.frames.last_mut().unwrap().0.get_mut(index).unwrap()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub fn peek(&self) -> Value {
        *self.stack.last().unwrap()
    }

    pub fn call(&mut self, frame_size: usize, module_index: usize, function_index: usize, params_count: usize) {
        self.frames.push(Frame::new(frame_size));
        self.module_indexes.push(module_index);
        self.function_indexes.push(function_index);
        self.opcode_indexes.push(0);
        self.params_counts.push(params_count);
    }

    pub fn params_count(&self) -> usize {
        *self.params_counts.last().unwrap()
    }

    pub fn ret(&mut self) {
        let result = self.pop();
        self.stack.resize(self.stack.len() - self.params_count(), Value::Bool(false));
        self.push(result);

        self.frames.pop();
        self.module_indexes.pop();
        self.function_indexes.pop();
        self.opcode_indexes.pop();
        self.params_counts.pop();
    }

    pub fn function_index(&self) -> usize {
        *self.function_indexes.last().unwrap()
    }

    pub fn module_index(&self) -> usize {
        *self.module_indexes.last().unwrap()
    }

    pub fn opcode_index(&self) -> usize {
        *self.opcode_indexes.last().unwrap()
    }

    pub fn opcode_index_mut(&mut self) -> &mut usize {
        self.opcode_indexes.last_mut().unwrap()
    }

    pub fn binary<T: BinaryOperator>(&mut self) -> Result<(), String> {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push(T::eval(lhs, rhs)?);
        Ok(())
    }
}
