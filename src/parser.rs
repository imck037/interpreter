use crate::lexer::{Lexer, Token};
use std::collections::HashMap;

pub fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression {
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
            Token::Assign => Expression::Operation("=".into(), Box::new(lhs), Box::new(rhs)),
            Token::Plus => Expression::Operation("+".into(), Box::new(lhs), Box::new(rhs)),
            Token::Minus => Expression::Operation("-".into(), Box::new(lhs), Box::new(rhs)),
            Token::Star => Expression::Operation("*".into(), Box::new(lhs), Box::new(rhs)),
            Token::Slash => Expression::Operation("/".into(), Box::new(lhs), Box::new(rhs)),
            Token::Caret => Expression::Operation("^".into(), Box::new(lhs), Box::new(rhs)),
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
pub enum Expression {
    Number(f32),
    Variable(String),
    Operation(String, Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn is_asign(&self) -> Option<(String, &Expression)> {
        match self {
            Expression::Operation(operator, left, right) if operator == "=" => {
                if let Expression::Variable(name) = &**left {
                    Some((name.to_string(), right))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn eval(&self, variables_table: &HashMap<String, f32>) -> f32 {
        match self {
            Expression::Number(c) => *c,

            Expression::Variable(var) => *variables_table.get(var).unwrap_or_else(|| {
                panic!("variable is undefined..");
            }),
            Expression::Operation(operator, lhs, rhs) => {
                let lhs = lhs.eval(variables_table);
                let rhs = rhs.eval(variables_table);
                match operator.as_str() {
                    "+" => return lhs + rhs,
                    "-" => return lhs - rhs,
                    "*" => return lhs * rhs,
                    "/" => return lhs / rhs,
                    "^" => return lhs.powf(rhs),
                    "=" =>  rhs,
                    op => panic!("Bad operator given: {}", op),
                }
            }
        }
    }
}
