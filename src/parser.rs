use std::ops::Range;

use crate::{
    common::Error,
    expression::{Binary, Expression, Literal, LiteralType},
    lexer::{PeekableTokenIterator, Token, TokenInfo},
    opcode::Opcode,
};

pub type ParseResult = Result<Expression, Error>;

fn end_of_file_location() -> std::ops::Range<usize> {
    0..0
}

fn unexpected_end() -> Result<TokenInfo, Error> {
    Err(Error {
        message: format!("Unexpected end of code."),
        location: end_of_file_location(),
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

fn parse_primary(it: &mut PeekableTokenIterator) -> ParseResult {
    let token_info = expect(it)?;

    match token_info.token {
        Token::Integer => Ok(Expression::Literal(Literal {
            literal_type: LiteralType::Integer,
            location: token_info.location,
        })),
        Token::Unknown => unknown(token_info.location),
        _ => unexpected(token_info.location),
    }
}

fn parse_factor(it: &mut PeekableTokenIterator) -> ParseResult {
    let mut lhs = parse_primary(it)?;
    while let Some(token_info) = it.peek() {
        match token_info.token {
            Token::Asterisk => {
                let location = it.next().unwrap().location;
                let rhs = parse_primary(it)?;
                lhs = Expression::Binary(Binary {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    opcode: Opcode::Multiply,
                    location,
                });
            }
            _ => break,
        }
    }
    Ok(lhs)
}

fn parse_term(it: &mut PeekableTokenIterator) -> ParseResult {
    let mut lhs = parse_factor(it)?;
    while let Some(token_info) = it.peek() {
        match token_info.token {
            Token::Plus => {
                let location = it.next().unwrap().location;
                let rhs = parse_factor(it)?;
                lhs = Expression::Binary(Binary {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    opcode: Opcode::Addict,
                    location,
                });
            }
            _ => break,
        }
    }
    Ok(lhs)
}

fn parse_expression(it: &mut PeekableTokenIterator) -> ParseResult {
    parse_term(it)
}

pub fn parse(it: &mut PeekableTokenIterator) -> ParseResult {
    let result = parse_expression(it)?;

    match it.next() {
        Some(token_info) => Err(Error {
            message: format!("Expected end, but found token."),
            location: token_info.location,
        }),
        None => Ok(result),
    }
}
