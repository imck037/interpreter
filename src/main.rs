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

        let number = Regex::new(r"[0-9]+(\.[0-9]?)").unwrap();
        let ident = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]+").unwrap();

        if input.is_empty() {
            return Token::Eof;
        }

        if let Some(m) = Regex::new(r"^/s+").unwrap().find(input) {
            self.pos += m.end();
            return self.next_token();
        }

        if let Some(m) = ident.find(input) {
            self.pos += m.end();
            return Token::Identifier(&input[..self.pos]);
        }
        if let Some(m) = number.find(input) {
            self.pos += m.end();
            return Token::Number(&input[..self.pos]);
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
}

// fn parse_expression<'a>(lexer: &mut Lexer<'a>, min_bp: f32) -> Expression<'a> {
//     let mut lhs = match lexer.next() {
//         Token::Operand(it) => Expression::Operand(it),
//         Token::Operator("(") => {
//             let lhs = parse_expression(lexer, 0.0);
//             assert_eq!(lexer.next(), Token::Operator(")"));
//             lhs
//         }
//         t => panic!("bad token: {:?}", t),
//     };
//     loop {
//         let op = match lexer.peek() {
//             Token::Eof => break,
//             Token::Operator(")") => break,
//             Token::Operator(op) => op,
//             t => panic!("bad token: {:?}", t),
//         };
//
//         let (l_bp, r_bp) = infix_binding_power(op);
//         if l_bp < min_bp {
//             break;
//         }
//         lexer.next();
//         let rhs = parse_expression(lexer, r_bp);
//         lhs = Expression::Operation(op, vec![lhs, rhs]);
//     }
//     lhs
// }
//
// fn infix_binding_power(operator: &str) -> (f32, f32) {
//     match operator {
//         "=" => (0.2, 0.1),
//         "+" | "-" => (1.0, 1.1),
//         "*" | "/" => (2.0, 2.1),
//         "^" | "√" => (3.1, 3.0),
//         "." => (4.0, 4.1),
//         _ => panic!("bad operator: {:?}", operator),
//     }
// }
//
// #[derive(Debug)]
// enum Expression<'a> {
//     Operand(&'a str),
//     Operation(&'a str, Vec<Expression<'a>>),
// }
//
// impl<'a> Expression<'a> {
//     fn from_input(input: &'a str) -> Expression<'a> {
//         let mut lexer = Lexer::new(&input);
//         parse_expression(&mut lexer, 0.0)
//     }
//     #[allow(unused)]
//     fn is_asign(&self) -> Option<(&'a str, &Expression<'a>)> {
//         match self {
//             Expression::Operand(_) => return None,
//             Expression::Operation(c, operands) => {
//                 if *c == "=" {
//                     let var_name = match operands.first().unwrap() {
//                         Expression::Operand(c) => c,
//                         _ => unreachable!(),
//                     };
//                     return Some((var_name, operands.last().unwrap()));
//                 }
//                 return None;
//             }
//         }
//     }
//     #[allow(unused)]
//     fn eval(&self, variables: &HashMap<String, f32>) -> f32 {
//         match self {
//             Expression::Operand(c) => {
//                 if let Ok(num) = c.parse::<f32>() {
//                     num
//                 } else {
//                     *variables.get(*c).unwrap()
//                 }
//             }
//             Expression::Operation(operator, operands) => {
//                 let lhs = operands.first().unwrap().eval(variables);
//                 let rhs = operands.last().unwrap().eval(variables);
//                 match *operator {
//                     "+" => return lhs + rhs,
//                     "-" => return lhs - rhs,
//                     "*" => return lhs * rhs,
//                     "/" => return lhs / rhs,
//                     "^" => return lhs.powf(rhs),
//                     "√" => return lhs.powf(1.0 / (rhs)),
//                     op => panic!("Bad operator: {}", op),
//                 }
//             }
//         }
//     }
// }
//

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
    }
}
