use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token<'a> {
    Operand(&'a str),
    Operator(&'a str),
    Eof,
}

struct Lexer<'a> {
    tokens: Vec<Token<'a>>,
}

#[allow(unused)]
impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        let mut tokens = input
            .split_whitespace()
            .map(|c| {
                if c.chars().all(|ch| ch.is_ascii_digit() || ch == '.')
                    || c.chars().all(|ch| ch.is_ascii_alphabetic())
                {
                    Token::Operand(c)
                } else {
                    Token::Operator(c)
                }
            })
            .collect::<Vec<_>>();
        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token<'a> {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&self) -> Token<'a> {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

fn parse_expression<'a>(lexer: &mut Lexer<'a>, min_bp: f32) -> Expression<'a> {
    let mut lhs = match lexer.next() {
        Token::Operand(it) => Expression::Operand(it),
        Token::Operator("(") => {
            let lhs = parse_expression(lexer, 0.0);
            assert_eq!(lexer.next(), Token::Operator(")"));
            lhs
        }
        t => panic!("bad token: {:?}", t),
    };
    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Operator(")") => break,
            Token::Operator(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        let (l_bp, r_bp) = infix_binding_power(op);
        if l_bp < min_bp {
            break;
        }
        lexer.next();
        let rhs = parse_expression(lexer, r_bp);
        lhs = Expression::Operation(op, vec![lhs, rhs]);
    }
    lhs
}

fn infix_binding_power(operator: &str) -> (f32, f32) {
    match operator {
        "=" => (0.2, 0.1),
        "+" | "-" => (1.0, 1.1),
        "*" | "/" => (2.0, 2.1),
        "^" | "√" => (3.1, 3.0),
        "." => (4.0, 4.1),
        _ => panic!("bad operator: {:?}", operator),
    }
}

#[derive(Debug)]
enum Expression<'a> {
    Operand(&'a str),
    Operation(&'a str, Vec<Expression<'a>>),
}

impl<'a> Expression<'a> {
    fn from_input(input: &'a str) -> Expression<'a> {
        let mut lexer = Lexer::new(&input);
        parse_expression(&mut lexer, 0.0)
    }
    #[allow(unused)]
    fn is_asign(&self) -> Option<(&'a str, &Expression<'a>)> {
        match self {
            Expression::Operand(_) => return None,
            Expression::Operation(c, operands) => {
                if *c == "=" {
                    let var_name = match operands.first().unwrap() {
                        Expression::Operand(c) => c,
                        _ => unreachable!(),
                    };
                    return Some((var_name, operands.last().unwrap()));
                }
                return None;
            }
        }
    }
    #[allow(unused)]
    fn eval(&self, variables: &HashMap<String, f32>) -> f32 {
        match self {
            Expression::Operand(c) => {
                if let Ok(num) = c.parse::<f32>() {
                    num
                } else {
                    *variables.get(*c).unwrap()
                }
            }
            Expression::Operation(operator, operands) => {
                let lhs = operands.first().unwrap().eval(variables);
                let rhs = operands.last().unwrap().eval(variables);
                match *operator {
                    "+" => return lhs + rhs,
                    "-" => return lhs - rhs,
                    "*" => return lhs * rhs,
                    "/" => return lhs / rhs,
                    "^" => return lhs.powf(rhs),
                    "√" => return lhs.powf(1.0 / (rhs)),
                    op => panic!("Bad operator: {}", op),
                }
            }
        }
    }
}

#[allow(unused)]
fn main() {
    let mut variables: HashMap<String, f32> = HashMap::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let expr = Expression::from_input(input.trim());
        if let Some((var_name, lhs)) = expr.is_asign() {
            let value = lhs.eval(&variables);
            variables.insert(var_name.to_string(), value);
            continue;
        }
        let value = expr.eval(&variables);
        println!("{}", value);
    }
}
