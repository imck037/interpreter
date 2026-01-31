enum Token {
    Atom(char),
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
                '0'..'9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Operator(c),
            })
            .collect::<Vec<_>>();
        Lexer { tokens }
    }
}

fn parse_expression(lexer: &Lexer) -> Expression {
    unimplemented!();
}

enum Expression {
    Atom(char),
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
