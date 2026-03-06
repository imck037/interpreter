use crate::lexer::{Lexer, Token};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EvalError {
    UndefinedVariable(String),
    BadOperator(String),
    BadToken,
}

pub fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Result<Expression, EvalError> {
    let mut lhs = match lexer.next_token() {
        Token::Identifier(id) => Expression::Variable(id.to_string()),
        Token::Number(num) => Expression::Number(num.parse().unwrap()),
        Token::LeftParenthesis => {
            let lhs = parse_expression(lexer, 0.0)?;
            if lexer.next_token() != Token::RightParenthesis {
                return Err(EvalError::BadToken);
            }
            lhs
        }
        _ => return Err(EvalError::BadToken),
    };
    loop {
        let op = lexer.peek();

        if op == Token::Eof || op == Token::RightParenthesis {
            break;
        }

        let (l_bp, r_bp) = infix_binding_power(op)?;
        if l_bp < min_bp {
            break;
        }

        lexer.next_token();

        let rhs = parse_expression(lexer, r_bp)?;

        lhs = match op {
            Token::Assign => Expression::Operation("=".into(), Box::new(lhs), Box::new(rhs)),
            Token::Plus => Expression::Operation("+".into(), Box::new(lhs), Box::new(rhs)),
            Token::Minus => Expression::Operation("-".into(), Box::new(lhs), Box::new(rhs)),
            Token::Star => Expression::Operation("*".into(), Box::new(lhs), Box::new(rhs)),
            Token::Slash => Expression::Operation("/".into(), Box::new(lhs), Box::new(rhs)),
            Token::Caret => Expression::Operation("^".into(), Box::new(lhs), Box::new(rhs)),
            _ => return Err(EvalError::BadToken),
        };
    }
    Ok(lhs)
}

fn infix_binding_power(operator: Token) -> Result<(f32, f32), EvalError> {
    match operator {
        Token::Assign => Ok((0.2, 0.1)),
        Token::Minus | Token::Plus => Ok((1.0, 1.1)),
        Token::Star | Token::Slash => Ok((2.0, 2.1)),
        Token::Caret => Ok((3.1, 3.0)),
        _ => Err(EvalError::BadOperator("syntax is wrong".to_string())),
    }
}

#[derive(Debug, Clone)]
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

    pub fn eval(&self, variables_table: &HashMap<String, f32>) -> Result<f32, EvalError> {
        match self {
            Expression::Number(c) => Ok(*c),

            Expression::Variable(var) => variables_table
                .get(var)
                .copied()
                .ok_or_else(|| EvalError::UndefinedVariable(var.clone())),
            Expression::Operation(operator, lhs, rhs) => {
                let lhs = lhs.eval(variables_table)?;
                let rhs = rhs.eval(variables_table)?;
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
