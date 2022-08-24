use std::{ops::Range, str::FromStr};

use crate::{
    builder::{self, Builder},
    common::Error,
    opcode::Opcode,
    value::{Real, Value},
};

#[derive(Clone, Copy)]
pub enum LiteralType {
    Integer,
    Real,
    True,
    False,
}

pub struct Literal {
    pub literal_type: LiteralType,
    pub location: std::ops::Range<usize>,
}

fn parse_u8_str<T: FromStr>(source: &[u8]) -> T {
    match std::str::from_utf8(source).unwrap().parse::<T>() {
        Ok(value) => value,
        Err(_) => panic!(),
    }
}

fn parse_integer(source: &[u8]) -> Value {
    Value::Integer(parse_u8_str(source))
}

fn parse_real(source: &[u8]) -> Value {
    Value::Real(Real(parse_u8_str(source)))
}

fn parse_literal(literal_type: LiteralType, source: &[u8]) -> Value {
    match literal_type {
        LiteralType::Integer => parse_integer(source),
        LiteralType::Real => parse_real(source),
        LiteralType::True => Value::Bool(true),
        LiteralType::False => Value::Bool(false),
    }
}

fn build_constant(value: Value, location: Range<usize>, builder: &mut Builder) {
    let index = builder.module_builder.push_constant(value);
    builder
        .function_builder
        .push(Opcode::Constant(index), location);
}

impl Literal {
    pub fn build(&self, builder: &mut Builder) -> Result<(), Error> {
        let source = &builder.module_builder.source[self.location.clone()];
        let value = parse_literal(self.literal_type, source);
        build_constant(value, self.location.clone(), builder);
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
