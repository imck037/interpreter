#[derive(Clone, Copy)]
enum Token {
    Operand(char),
    Operator(char),
    Eof,
}

struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|item| !item.is_ascii_whitespace())
            .map(|c| match c {
                '0'..'9' | 'a'..='z' | 'A'..='Z' => Token::Operand(c),
                _ => Token::Operator(c),
            })
            .collect::<Vec<_>>();
        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

fn parse_expression(lexer: &Lexer) -> Expression {
    unimplemented!();
}

enum Expression {
    Operand(char),
    Operation(char, Vec<Expression>),
}

impl Expression {
    fn from_input(input: &str) -> Expression {
        let mut lexer = Lexer::new(input);
        parse_expression(&lexer)
    }
}

fn main() {
    println!("Hello, world!");
}
