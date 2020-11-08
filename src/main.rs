#![warn(rust_2018_idioms)]
mod parser;
mod tokeniser;
mod repl;

use std::io::{self, Write};

use parser::{Parser};
use repl::evaluate_expression;
use tokeniser::Tokeniser;


fn main() {
    let stdin = io::stdin();
    
    loop {
        let mut user_expr = String::new();
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout.");
        match stdin.read_line(&mut user_expr) {
            Ok(_) => {},
            Err(_) => {
                println!("Failed to read input.");
                continue;
            }
        };

        let tokens = Tokeniser::new()
            .from_string(String::from(user_expr.clone()))
            .get_tokens();
        let parsed_expressions = Parser::new().with_tokens(tokens).parse();
        // println!("{:#?}", parsed_expressions);

        let result = evaluate_expression(&parsed_expressions[0]);

        println!("{}", result);
    }
}


