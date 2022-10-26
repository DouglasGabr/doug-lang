use std::io::{self, Write};

mod ast;
mod lexer;
mod parser;

fn main() {
    println!("Repl v1.0");
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().ok();
        io::stdin().read_line(&mut input).unwrap();
        if input.contains("exit") || input.trim().is_empty() {
            break;
        }
        let mut parser = parser::Parser::new(&input);
        let program = parser.produce_ast();
        println!("{:#?}", program);
    }
}
