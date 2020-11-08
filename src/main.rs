#![warn(rust_2018_idioms)]
mod parser;
mod tokeniser;

use parser::{Parser};
use tokeniser::Tokeniser;



fn main() {
    let tokens = Tokeniser::new()
        .from_string(String::from("1 + 2 * 3 - 3 / 4"))
        .get_tokens();
    
    let parsed_expressions = Parser::new().with_tokens(tokens).parse();
    println!("{:#?}", parsed_expressions);
}


