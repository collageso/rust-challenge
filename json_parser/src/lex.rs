use std::str::Chars;

pub enum Token {
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    Comma,
    Colon,
    Null,
    True,
    False,
    Number(f64),
    String(String),
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
        }
    }

    pub fn tokenize() {}

    fn next_token() {}
}
