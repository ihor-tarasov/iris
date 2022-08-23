use std::iter::{Cloned, Enumerate, Peekable};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Integer,
    Plus,     // +
    Asterisk, // *
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

impl<'a> TokenIterator<'a> {
    pub fn new(source: &'a str) -> Self {
        Self(source.as_bytes().iter().cloned().enumerate().peekable())
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
        while let Some(&(_, c)) = self.0.peek() {
            if c.is_ascii_digit() {
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
                token: Token::Integer,
                location: begin..end,
            })
        }
    }

    fn read_simple(&mut self) -> Option<TokenInfo> {
        let (begin, c) = self.0.next()?;
        let token = match c {
            b'+' => Token::Plus,
            b'*' => Token::Asterisk,
            _ => Token::Unknown,
        };
        Some(TokenInfo {
            token,
            location: begin..(begin + 1),
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
