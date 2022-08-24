use std::ops::Range;

use crate::{
    builder::{self, Builder},
    common::Error,
    opcode::Opcode,
    value::Value,
};

pub struct Literal {
    pub value: Value,
    pub location: std::ops::Range<usize>,
}

fn build_constant(value: Value, location: Range<usize>, builder: &mut Builder) {
    let index = builder.module_builder.push_constant(value);
    builder
        .function_builder
        .push(Opcode::Constant(index), location);
}

impl Literal {
    pub fn build(&self, builder: &mut Builder) -> Result<(), Error> {
        build_constant(self.value, self.location.clone(), builder);
        Ok(())
    }
}

pub struct Binary {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub opcode: Opcode,
    pub location: Range<usize>,
}

impl Binary {
    pub fn build(&self, builder: &mut Builder) -> Result<(), Error> {
        builder::build(&self.lhs, builder)?;
        builder::build(&self.rhs, builder)?;
        builder
            .function_builder
            .push(self.opcode, self.location.clone());
        Ok(())
    }
}

pub enum BinaryLogicType {
    And,
    Or,
}

pub struct BinaryLogic {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub logic_type: BinaryLogicType,
    pub location: std::ops::Range<usize>,
}

/*
a && b

    {a}
    jump_false set_false
    {b}
    jump expr_end
set_false:
    push_false
expr_end:

a || b

    {a}
    jump_true set_true
    {b}
    jump expr_end
set_true:
    push_true
expr_end:
*/
impl BinaryLogic {
    pub fn build(&self, builder: &mut Builder) -> Result<(), Error> {
        builder::build(&self.lhs, builder)?;
        let set_addr = builder.function_builder.push_unknown(self.location.clone());
        builder::build(&self.rhs, builder)?;
        let expr_end_addr = builder.function_builder.push_unknown(self.location.clone());
        builder.function_builder.set(
            set_addr,
            match self.logic_type {
                BinaryLogicType::And => Opcode::JumpFalse(builder.function_builder.len()),
                BinaryLogicType::Or => Opcode::JumpTrue(builder.function_builder.len()),
            },
        );
        build_constant(
            Value::Bool(match self.logic_type {
                BinaryLogicType::And => false,
                BinaryLogicType::Or => true,
            }),
            self.location.clone(),
            builder,
        );
        builder
            .function_builder
            .set(expr_end_addr, Opcode::Jump(builder.function_builder.len()));
        Ok(())
    }
}

pub enum Expression {
    Literal(Literal),
    Binary(Binary),
    BinaryLogic(BinaryLogic),
}
