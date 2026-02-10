use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'a> {
    Identifier(&'a str),
    Number(&'a str),
    Plus,
    Minus,
    Slash,
    Star,
    Caret,
    Assign,
    LeftParenthesis,
    RightParenthesis,
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

#[allow(unused)]
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        let input = &self.input[self.pos..];

        let number = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        let ident = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();

        if input.is_empty() {
            return Token::Eof;
        }

        if let Some(m) = Regex::new(r"^\s+").unwrap().find(input) {
            self.pos += m.end();
            return self.next_token();
        }

        if let Some(m) = ident.find(input) {
            self.pos += m.end();
            return Token::Identifier(&input[..m.end()]);
        }

        if let Some(m) = number.find(input) {
            self.pos += m.end();
            return Token::Number(&input[..m.end()]);
        }

        let ch = input.chars().next().unwrap();
        self.pos += ch.len_utf8();

        match ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '=' => Token::Assign,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '^' => Token::Caret,
            _ => panic!("Unknown character..."),
        }
    }

    pub fn peek(&mut self) -> Token<'a> {
        let saved = self.pos;
        let t = self.next_token();
        self.pos = saved;
        t
    }
}
