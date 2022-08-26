use std::ops::Range;

use crate::{
    builder::{self, Builder},
    common::Error,
    program::Opcode,
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
    JumpFalse set_false
    {b}
    Jump expr_end
set_false:
    Constant false
expr_end:

a || b

    {a}
    JumpTrue set_true
    {b}
    Jump expr_end
set_true:
    Constant true
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

fn get_local(builder: &mut Builder, name: &String, location: &Range<usize>) -> Result<usize, Error> {
    match builder.function_builder.get_local(name) {
        Some(index) => Ok(index),
        None => {
            return Err(Error {
                message: format!("Can't find variable \"{}\".", name),
                location: location.clone(),
            })
        }
    }
}

pub struct Variable {
    pub name: String,
    pub location: Range<usize>,
}

impl Variable {
    pub fn build(&self, builder: &mut Builder) -> Result<(), Error> {
        let position = get_local(builder, &self.name, &self.location)?;

        builder
            .function_builder
            .push(Opcode::LoadLocal(position), self.location.clone());
        Ok(())
    }
}

pub struct Assignment {
    pub name: String,
    pub expr: Box<Expression>,
    pub create_new_variable: bool,
    pub location: Range<usize>,
}

impl Assignment {
    pub fn build(&self, builder: &mut Builder) -> Result<(), Error> {
        let position = if self.create_new_variable {
            builder.function_builder.new_local(&self.name)
        } else {
            get_local(builder, &self.name, &self.location)?
        };

        builder::build(&self.expr, builder)?;
        builder
            .function_builder
            .push(Opcode::Push, self.location.clone());
        builder
            .function_builder
            .push(Opcode::StoreLocal(position), self.location.clone());

        Ok(())
    }
}

pub enum Expression {
    Literal(Literal),
    Binary(Binary),
    BinaryLogic(BinaryLogic),
    Variable(Variable),
    Assignment(Assignment),
}
