use std::{
    collections::HashMap,
    io::{self, Write},
};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token<'a> {
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

struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

#[allow(unused)]
impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input, pos: 0 }
    }

    fn next_token(&mut self) -> Token<'a> {
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

    fn peek(&mut self) -> Token<'a> {
        let saved = self.pos;
        let t = self.next_token();
        self.pos = saved;
        t
    }
}

fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression {
    let mut lhs = match lexer.next_token() {
        Token::Identifier(id) => Expression::Variable(id.to_string()),
        Token::Number(num) => Expression::Number(num.parse().unwrap()),
        Token::LeftParenthesis => {
            let lhs = parse_expression(lexer, 0.0);
            assert_eq!(lexer.next_token(), Token::RightParenthesis);
            lhs
        }
        t => panic!("bad token: {:?}", t),
    };
    loop {
        let op = lexer.peek();

        if op == Token::Eof || op == Token::RightParenthesis {
            break;
        }

        let (l_bp, r_bp) = infix_binding_power(op);
        if l_bp < min_bp {
            break;
        }

        lexer.next_token();

        let rhs = parse_expression(lexer, r_bp);

        lhs = match op {
            Token::Plus => Expression::Operation("+".into(), vec![lhs, rhs]),
            Token::Minus => Expression::Operation("-".into(), vec![lhs, rhs]),
            Token::Star => Expression::Operation("*".into(), vec![lhs, rhs]),
            Token::Slash => Expression::Operation("/".into(), vec![lhs, rhs]),
            Token::Assign => Expression::Operation("=".into(), vec![lhs, rhs]),
            Token::Caret => Expression::Operation("^".into(), vec![lhs, rhs]),
            t => panic!("Unexpected token: {:?}", t),
        };
    }
    lhs
}

fn infix_binding_power(operator: Token) -> (f32, f32) {
    match operator {
        Token::Assign => (0.2, 0.1),
        Token::Minus | Token::Plus => (1.0, 1.1),
        Token::Star | Token::Slash => (2.0, 2.1),
        Token::Caret => (3.1, 3.0),
        _ => panic!("bad operator: {:?}", operator),
    }
}

#[derive(Debug)]
enum Expression {
    Number(f32),
    Variable(String),
    Operation(String, Vec<Expression>),
}

impl Expression {
    #[allow(unused)]
    fn is_asign(&self) -> Option<(String, &Expression)> {
        match self {
            Expression::Operation(c, operands) => {
                if c == "=" {
                    let var_name = match operands.first().unwrap() {
                        Expression::Variable(c) => c,
                        _ => unreachable!(),
                    };
                    return Some((var_name.to_string(), operands.last().unwrap()));
                }
                return None;
            }
            _ => None,
        }
    }

    #[allow(unused)]
    fn eval(&self, variables_table: &HashMap<String, f32>) -> f32 {
        match self {
            Expression::Number(c) => *c,

            Expression::Variable(var) => *variables_table.get(var).unwrap_or_else(|| {
                panic!("variable is undefined..");
            }),
            Expression::Operation(operator, operands) => {
                let lhs = operands.first().unwrap().eval(variables_table);
                let rhs = operands.last().unwrap().eval(variables_table);
                match operator.as_str() {
                    "+" => return lhs + rhs,
                    "-" => return lhs - rhs,
                    "*" => return lhs * rhs,
                    "/" => return lhs / rhs,
                    "^" => return lhs.powf(rhs),
                    "√" => return lhs.powf(1.0 / (rhs)),
                    op => panic!("Bad operator given: {}", op),
                }
            }
        }
    }
}

#[allow(unused)]
fn main() {
    let mut variables_table: HashMap<String, f32> = HashMap::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let mut lexer = Lexer::new(&input);
        let expression = parse_expression(&mut lexer, 0.0);
        if let Some((var, lhs)) = expression.is_asign() {
            let value = lhs.eval(&variables_table);
            variables_table.insert(var.to_string(), value);
        }
        let value = expression.eval(&variables_table);
        println!("{}", value);
    }
}
