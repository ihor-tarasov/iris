use crate::{
    common::Error, function::FunctionBuilder, module::ModuleBuilder, opcode::Opcode, value::Value,
};

pub enum LiteralType {
    Integer,
}

pub struct Literal {
    pub literal_type: LiteralType,
    pub location: std::ops::Range<usize>,
}

fn parse_integer(source: &[u8]) -> Value {
    Value::Integer(std::str::from_utf8(source).unwrap().parse::<i64>().unwrap())
}

impl Literal {
    pub fn build(
        &self,
        module_builder: &mut ModuleBuilder,
        function_builder: &mut FunctionBuilder,
    ) -> Result<(), Error> {
        let source = &module_builder.source[self.location.clone()];
        let value = match self.literal_type {
            LiteralType::Integer => parse_integer(source),
        };
        let constant_index = module_builder.push_constant(value);
        function_builder.push(Opcode::Constant(constant_index), self.location.clone());
        Ok(())
    }
}

pub struct Binary {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub opcode: Opcode,
    pub location: std::ops::Range<usize>,
}

impl Binary {
    fn build(
        &self,
        module_builder: &mut ModuleBuilder,
        function_builder: &mut FunctionBuilder,
    ) -> Result<(), Error> {
        build(&self.lhs, module_builder, function_builder)?;
        build(&self.rhs, module_builder, function_builder)?;
        function_builder.push(self.opcode, self.location.clone());
        Ok(())
    }
}

pub enum Expression {
    Literal(Literal),
    Binary(Binary),
}

pub fn build(
    expression: &Expression,
    module_builder: &mut ModuleBuilder,
    function_builder: &mut FunctionBuilder,
) -> Result<(), Error> {
    match expression {
        Expression::Literal(literal) => literal.build(module_builder, function_builder),
        Expression::Binary(binary) => binary.build(module_builder, function_builder),
    }
}
