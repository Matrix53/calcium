mod parser;
mod reader;
mod token;
mod tokenizer;

use parser::Parser;
use tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let source = std::fs::read_to_string(&args[1]).unwrap();
    let tokens = Tokenizer::tokenize(&source);
    let output = Parser::parse(tokens);
    std::fs::write(&args[2], output).unwrap();
}
