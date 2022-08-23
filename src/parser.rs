use std::ops::Range;

use crate::{
    common::Error,
    expression::{Binary, Expression, Literal, LiteralType},
    lexer::{PeekableTokenIterator, Token, TokenInfo},
    opcode::Opcode,
};

pub type ParseResult = Result<Expression, Error>;

fn end_of_file_location() -> Range<usize> {
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
        Token::Real => Ok(Expression::Literal(Literal {
            literal_type: LiteralType::Real,
            location: token_info.location,
        })),
        Token::Unknown => unknown(token_info.location),
        _ => unexpected(token_info.location),
    }
}

fn term_mapper(token: Token) -> Option<Opcode> {
    match token {
        Token::Plus => Some(Opcode::Addict),
        _ => None,
    }
}

fn factor_mapper(token: Token) -> Option<Opcode> {
    match token {
        Token::Asterisk => Some(Opcode::Multiply),
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

fn parse_factor(it: &mut PeekableTokenIterator) -> ParseResult {
    parse_binary(it, parse_primary, factor_mapper)
}

fn parse_term(it: &mut PeekableTokenIterator) -> ParseResult {
    parse_binary(it, parse_factor, term_mapper)
}

fn parse_expression(it: &mut PeekableTokenIterator) -> ParseResult {
    parse_term(it)
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
