use std::{
    collections::HashMap,
    io::{self, Write},
};

mod lexer;
mod parser;

use lexer::Lexer;
use parser::parse_expression;

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
            variables_table.insert(var.to_string(), value.unwrap());
        }
        match expression.eval(&variables_table) {
            Ok(value) => {
                println!("{value}");
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
