use std::{iter::Peekable, ops::Range};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Integer,
    Real,
    Identifier,
    True,
    False,
    Let,
    Plus,                   // +
    Minus,                  // -
    Asterisk,               // *
    Slash,                  // /
    Percent,                // %
    LessLess,               // <<
    GreaterGreater,         // >>
    Ampersand,              // &
    VerticalBar,            // |
    Circumflex,             // ^
    Less,                   // <
    Greater,                // >
    Equal,                  // =
    Comma,                  // ,
    LessEqual,              // <=
    GreaterEqual,           // >=
    EqualEqual,             // ==
    ExclamationEqual,       // !=
    AmpersandAmpersand,     // &&
    VerticalBarVerticalBar, // ||
    Unknown,
}

#[derive(Debug)]
pub struct TokenInfo {
    pub token: Token,
    pub location: std::ops::Range<usize>,
}

pub struct CodeIterator<'a> {
    source: &'a [u8],
    position: usize,
}

impl<'a> CodeIterator<'a> {
    fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.position).cloned()
    }

    fn skip(&mut self) {
        self.position += 1;
    }
}

pub struct TokenIterator<'a>(CodeIterator<'a>);

fn is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\r' || c == b'\n'
}

fn find_keyword(word: &[u8]) -> Option<Token> {
    match word {
        b"false" => Some(Token::False),
        b"true" => Some(Token::True),
        b"let" => Some(Token::Let),
        _ => None,
    }
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
        b'=' => Token::Equal,
        b',' => Token::Comma,
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
        (b'&', b'&') => Some(Token::AmpersandAmpersand),
        (b'|', b'|') => Some(Token::VerticalBarVerticalBar),
        _ => None,
    }
}

impl<'a> TokenIterator<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self(CodeIterator::new(source))
    }

    fn skip_whitespaces(&mut self) -> Option<()> {
        loop {
            let c = self.0.peek()?;
            if is_whitespace(c) {
                self.0.skip();
            } else {
                break Some(());
            }
        }
    }

    fn read_number(&mut self) -> Option<TokenInfo> {
        let begin = self.0.position;
        let mut is_real = false;
        while let Some(c) = self.0.peek() {
            if c.is_ascii_digit() || c == b'.' {
                if c == b'.' {
                    if is_real {
                        break;
                    } else {
                        is_real = true;
                    }
                }
                self.0.skip();
            } else {
                break;
            }
        }
        if begin == self.0.position {
            None
        } else {
            Some(TokenInfo {
                token: if is_real { Token::Real } else { Token::Integer },
                location: begin..self.0.position,
            })
        }
    }

    fn read_identifier(&mut self) -> Option<TokenInfo> {
        let begin = self.0.position;
        while let Some(c) = self.0.peek() {
            if c.is_ascii_alphanumeric() || c == b'_' {
                self.0.skip();
            } else {
                break;
            }
        }
        if begin == self.0.position {
            None
        } else {
            let location = begin..self.0.position;
            Some(TokenInfo {
                token: find_keyword(&self.0.source[location.clone()]).unwrap_or(Token::Identifier),
                location,
            })
        }
    }

    fn read_simple(&mut self) -> Option<TokenInfo> {
        let begin = self.0.position;
        let c1 = self.0.peek()?;
        self.0.skip();

        if let Some(c2) = self.0.peek() {
            if let Some(token) = parse_double_token(c1, c2) {
                self.0.skip();
                return Some(TokenInfo {
                    token,
                    location: begin..(begin + 2),
                });
            }
        }

        Some(TokenInfo {
            token: parse_single_token(c1),
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
        if let Some(token) = self.read_identifier() {
            return Some(token);
        }
        self.read_simple()
    }
}

pub struct PeekableTokenIterator<'a> {
    it: Peekable<TokenIterator<'a>>,
    source: &'a [u8],
}

impl<'a> PeekableTokenIterator<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            it: TokenIterator::new(source).peekable(),
            source,
        }
    }

    pub fn peek(&mut self) -> Option<&TokenInfo> {
        self.it.peek()
    }

    pub fn next(&mut self) -> Option<TokenInfo> {
        self.it.next()
    }

    pub fn slice(&self, location: Range<usize>) -> &[u8] {
        &self.source[location]
    }
}
