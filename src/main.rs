use std::io::{self, Write};

use crate::{
    frontend::{ast::Statement, parser::Parser},
    runtime::{environment::Environment, interpreter::evaluate, values::RuntimeValue},
};

mod frontend;
mod runtime;

fn main() {
    println!("Repl v1.0");
    let mut environment = Environment::new(None);
    environment.declare_variable("null".to_owned(), RuntimeValue::Null);
    environment.declare_variable("true".to_owned(), RuntimeValue::Boolean(true));
    environment.declare_variable("false".to_owned(), RuntimeValue::Boolean(false));
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().ok();
        io::stdin().read_line(&mut input).unwrap();
        if input.contains("exit") || input.trim().is_empty() {
            break;
        }
        let mut parser = Parser::new(&input);
        let program = parser.produce_ast();
        println!("{:?}", program);
        let result = evaluate(Statement::Program(program), &mut environment);
        println!("{:?}", result);
    }
}
