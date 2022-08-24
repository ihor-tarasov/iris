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

pub enum Expression {
    Literal(Literal),
    Binary(Binary),
}
