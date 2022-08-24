use std::iter::{Cloned, Enumerate, Peekable};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Integer,
    Real,
    Plus,             // +
    Minus,            // -
    Asterisk,         // *
    Slash,            // /
    Percent,          // %
    LessLess,         // <<
    GreaterGreater,   // >>
    Ampersand,        // &
    VerticalBar,      // |
    Circumflex,       // ^
    Less,             // <
    Greater,          // >
    LessEqual,        // <=
    GreaterEqual,     // >=
    EqualEqual,       // ==
    ExclamationEqual, // !=
    Unknown,
}

#[derive(Debug)]
pub struct TokenInfo {
    pub token: Token,
    pub location: std::ops::Range<usize>,
}

pub struct TokenIterator<'a>(Peekable<Enumerate<Cloned<std::slice::Iter<'a, u8>>>>);

fn is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\r' || c == b'\n'
}

fn parse_single_token(c: u8) -> Token {
    match c {
        b'+' => Token::Plus,
        b'-' => Token::Minus,
        b'*' => Token::Asterisk,
        b'/' => Token::Slash,
        b'%' => Token::Percent,
        b'&' => Token::Ampersand,
        b'|' => Token::VerticalBar,
        b'^' => Token::Circumflex,
        b'<' => Token::Less,
        b'>' => Token::Greater,
        _ => Token::Unknown,
    }
}

fn parse_double_token(c1: u8, c2: u8) -> Option<Token> {
    match (c1, c2) {
        (b'>', b'>') => Some(Token::GreaterGreater),
        (b'<', b'<') => Some(Token::LessLess),
        (b'=', b'=') => Some(Token::EqualEqual),
        (b'!', b'=') => Some(Token::ExclamationEqual),
        (b'>', b'=') => Some(Token::GreaterEqual),
        (b'<', b'=') => Some(Token::LessEqual),
        _ => None,
    }
}

impl<'a> TokenIterator<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self(source.iter().cloned().enumerate().peekable())
    }

    fn skip_whitespaces(&mut self) -> Option<()> {
        loop {
            let (_, c) = *self.0.peek()?;
            if is_whitespace(c) {
                self.0.next().unwrap();
            } else {
                break Some(());
            }
        }
    }

    fn read_number(&mut self) -> Option<TokenInfo> {
        let (begin, _) = *self.0.peek()?;
        let mut end = begin;
        let mut is_real = false;
        while let Some(&(_, c)) = self.0.peek() {
            if c.is_ascii_digit() || c == b'.' {
                if c == b'.' {
                    if is_real {
                        break;
                    } else {
                        is_real = true;
                    }
                }
                self.0.next().unwrap();
                end += 1;
            } else {
                break;
            }
        }
        if begin == end {
            None
        } else {
            Some(TokenInfo {
                token: if is_real { Token::Real } else { Token::Integer },
                location: begin..end,
            })
        }
    }

    fn read_simple(&mut self) -> Option<TokenInfo> {
        let (begin, c1) = self.0.next()?;
        let (_, c2) = *self.0.peek().unwrap_or(&(0, b'\0'));
        Some(match parse_double_token(c1, c2) {
            Some(token) => {
                self.0.next().unwrap();
                TokenInfo {
                    token,
                    location: begin..(begin + 2),
                }
            }
            None => TokenInfo {
                token: parse_single_token(c1),
                location: begin..(begin + 2),
            },
        })
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = TokenInfo;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces()?;
        if let Some(token) = self.read_number() {
            return Some(token);
        }
        self.read_simple()
    }
}

pub type PeekableTokenIterator<'a> = Peekable<TokenIterator<'a>>;
