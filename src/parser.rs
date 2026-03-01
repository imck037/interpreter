use crate::lexer::{Lexer, Token};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EvalError<'a> {
    UndefinedVariable(String),
    BadOperator(String),
    BadToken(Token<'a>),
}

pub fn parse_expression<'a>(
    lexer: &mut Lexer,
    min_bp: f32,
) -> Result<Expression<'a>, EvalError<'a>> {
    let mut lhs = match lexer.next_token() {
        Token::Identifier(id) => Ok(Expression::Variable(id.to_string())),
        Token::Number(num) => Ok(Expression::Number(num.parse().unwrap())),
        Token::LeftParenthesis => {
            let lhs = parse_expression(lexer, 0.0);
            assert_eq!(lexer.next_token(), Token::RightParenthesis);
            lhs
        }
        t => Err(EvalError::BadToken(t)),
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
            Token::Assign => Ok(Expression::Operation(
                "=".into(),
                Box::new(lhs),
                Box::new(rhs),
            )),
            Token::Plus => Ok(Expression::Operation(
                "+".into(),
                Box::new(lhs),
                Box::new(rhs),
            )),
            Token::Minus => Ok(Expression::Operation(
                "-".into(),
                Box::new(lhs),
                Box::new(rhs),
            )),
            Token::Star => Ok(Expression::Operation(
                "*".into(),
                Box::new(lhs),
                Box::new(rhs),
            )),
            Token::Slash => Ok(Expression::Operation(
                "/".into(),
                Box::new(lhs),
                Box::new(rhs),
            )),
            Token::Caret => Ok(Expression::Operation(
                "^".into(),
                Box::new(lhs),
                Box::new(rhs),
            )),
            t => Err(EvalError::BadToken(t)),
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

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Number(f32),
    Variable(String),
    Operation(
        String,
        Box<Result<Expression<'a>, EvalError<'a>>>,
        Box<Result<Expression<'a>, EvalError<'a>>>,
    ),
}

impl<'a> Expression<'a> {
    pub fn is_asign(&self) -> Option<(String, &Expression)> {
        match self {
            Expression::Operation(operator, left, right) if operator == "=" => {
                if let Ok(Expression::Variable(name)) = &**left {
                    Some((name.to_string(), &right.unwrap()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn eval(&self, variables_table: &HashMap<String, f32>) -> Result<f32, EvalError> {
        match self {
            Expression::Number(c) => Ok(*c),

            Expression::Variable(var) => variables_table
                .get(var)
                .copied()
                .ok_or_else(|| EvalError::UndefinedVariable(var.clone())),
            Expression::Operation(operator, lhs, rhs) => {
                let lhs = lhs.unwrap().eval(variables_table)?;
                let rhs = rhs.unwrap().eval(variables_table)?;
                match operator.as_str() {
                    "+" => return Ok(lhs + rhs),
                    "-" => return Ok(lhs - rhs),
                    "*" => return Ok(lhs * rhs),
                    "/" => return Ok(lhs / rhs),
                    "^" => return Ok(lhs.powf(rhs)),
                    "=" => Ok(rhs),
                    op => Err(EvalError::BadOperator(op.to_string())),
                }
            }
        }
    }
}
