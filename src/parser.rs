use std::{ops::Range, str::FromStr};

use crate::{
    common::Error,
    expression::{Binary, BinaryLogic, BinaryLogicType, Expression, Literal},
    lexer::{PeekableTokenIterator, Token, TokenInfo},
    program::Opcode,
    value::Value,
};

pub type ParseResult = Result<Expression, Error>;

fn unexpected_end() -> Result<TokenInfo, Error> {
    Err(Error {
        message: format!("Unexpected end of code."),
        location: 0..0,
    })
}

fn expect(it: &mut PeekableTokenIterator) -> Result<TokenInfo, Error> {
    match it.next() {
        Some(token_info) => Ok(token_info),
        None => unexpected_end(),
    }
}

fn unexpected(location: Range<usize>) -> ParseResult {
    Err(Error {
        message: format!("Unexpected token."),
        location,
    })
}

fn unknown(location: Range<usize>) -> ParseResult {
    Err(Error {
        message: format!("Unknown character."),
        location,
    })
}

fn parse_u8_str<T: FromStr>(
    it: &mut PeekableTokenIterator,
    location: Range<usize>,
) -> Result<T, Error> {
    std::str::from_utf8(it.slice(location.clone()))
        .unwrap()
        .parse::<T>()
        .map_err(|_| Error {
            message: format!("Unable to parse literal value may be it so long."),
            location,
        })
}

fn create_literal(value: Value, location: Range<usize>) -> ParseResult {
    Ok(Expression::Literal(Literal { value, location }))
}

fn parse_integer(it: &mut PeekableTokenIterator, location: Range<usize>) -> ParseResult {
    create_literal(
        Value::Integer(parse_u8_str(it, location.clone())?),
        location,
    )
}

fn parse_real(it: &mut PeekableTokenIterator, location: Range<usize>) -> ParseResult {
    create_literal(
        Value::Real(parse_u8_str(it, location.clone())?),
        location,
    )
}

fn parse_primary(it: &mut PeekableTokenIterator) -> ParseResult {
    let token_info = expect(it)?;

    match token_info.token {
        Token::Integer => parse_integer(it, token_info.location),
        Token::Real => parse_real(it, token_info.location),
        Token::True => create_literal(Value::Bool(true), token_info.location),
        Token::False => create_literal(Value::Bool(false), token_info.location),
        Token::Unknown => unknown(token_info.location),
        _ => unexpected(token_info.location),
    }
}

fn and_mapper(token: Token) -> Option<BinaryLogicType> {
    if token == Token::AmpersandAmpersand {
        Some(BinaryLogicType::And)
    } else {
        None
    }
}

fn or_mapper(token: Token) -> Option<BinaryLogicType> {
    if token == Token::VerticalBarVerticalBar {
        Some(BinaryLogicType::Or)
    } else {
        None
    }
}

fn equality_mapper(token: Token) -> Option<Opcode> {
    match token {
        Token::EqualEqual => Some(Opcode::Equal),
        Token::ExclamationEqual => Some(Opcode::NotEqual),
        _ => None,
    }
}

fn comparison_mapper(token: Token) -> Option<Opcode> {
    match token {
        Token::Less => Some(Opcode::Less),
        Token::LessEqual => Some(Opcode::LessEqual),
        Token::Greater => Some(Opcode::Greater),
        Token::GreaterEqual => Some(Opcode::GreaterEqual),
        _ => None,
    }
}

fn term_mapper(token: Token) -> Option<Opcode> {
    match token {
        Token::Plus => Some(Opcode::Addict),
        Token::Minus => Some(Opcode::Subtract),
        _ => None,
    }
}

fn factor_mapper(token: Token) -> Option<Opcode> {
    match token {
        Token::Asterisk => Some(Opcode::Multiply),
        Token::Slash => Some(Opcode::Divide),
        Token::Percent => Some(Opcode::Modulo),
        _ => None,
    }
}

fn bitwise_mapper(token: Token) -> Option<Opcode> {
    match token {
        Token::Ampersand => Some(Opcode::And),
        Token::VerticalBar => Some(Opcode::Or),
        Token::Circumflex => Some(Opcode::Xor),
        Token::GreaterGreater => Some(Opcode::Shr),
        Token::LessLess => Some(Opcode::Shl),
        _ => None,
    }
}

fn parse_binary(
    it: &mut PeekableTokenIterator,
    next: fn(&mut PeekableTokenIterator) -> ParseResult,
    mapper: fn(Token) -> Option<Opcode>,
) -> ParseResult {
    let mut lhs = (next)(it)?;
    while let Some(token_info) = it.peek() {
        if let Some(opcode) = (mapper)(token_info.token) {
            let location = it.next().unwrap().location;
            let rhs = (next)(it)?;
            lhs = Expression::Binary(Binary {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                opcode,
                location,
            });
        } else {
            break;
        }
    }
    Ok(lhs)
}

fn parse_binary_logic(
    it: &mut PeekableTokenIterator,
    next: fn(&mut PeekableTokenIterator) -> ParseResult,
    mapper: fn(Token) -> Option<BinaryLogicType>,
) -> ParseResult {
    let mut lhs = (next)(it)?;
    while let Some(token_info) = it.peek() {
        if let Some(logic_type) = (mapper)(token_info.token) {
            let location = it.next().unwrap().location;
            let rhs = (next)(it)?;
            lhs = Expression::BinaryLogic(BinaryLogic {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                logic_type,
                location,
            });
        } else {
            break;
        }
    }
    Ok(lhs)
}

fn parse_bitwise(it: &mut PeekableTokenIterator) -> ParseResult {
    parse_binary(it, parse_primary, bitwise_mapper)
}

fn parse_factor(it: &mut PeekableTokenIterator) -> ParseResult {
    parse_binary(it, parse_bitwise, factor_mapper)
}

fn parse_term(it: &mut PeekableTokenIterator) -> ParseResult {
    parse_binary(it, parse_factor, term_mapper)
}

fn parse_comparison(it: &mut PeekableTokenIterator) -> ParseResult {
    parse_binary(it, parse_term, comparison_mapper)
}

fn parse_equality(it: &mut PeekableTokenIterator) -> ParseResult {
    parse_binary(it, parse_comparison, equality_mapper)
}

fn parse_binary_and(it: &mut PeekableTokenIterator) -> ParseResult {
    parse_binary_logic(it, parse_equality, and_mapper)
}

fn parse_binary_or(it: &mut PeekableTokenIterator) -> ParseResult {
    parse_binary_logic(it, parse_binary_and, or_mapper)
}

fn parse_expression(it: &mut PeekableTokenIterator) -> ParseResult {
    parse_binary_or(it)
}

pub fn parse(it: &mut PeekableTokenIterator) -> ParseResult {
    let result = parse_expression(it)?;

    match it.next() {
        Some(token_info) => {
            if token_info.token == Token::Unknown {
                unknown(token_info.location)
            } else {
                Err(Error {
                    message: format!("Expected end, but found token."),
                    location: token_info.location,
                })
            }
        }
        None => Ok(result),
    }
}
