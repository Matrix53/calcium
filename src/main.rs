mod parser;
mod reader;
mod token;
mod tokenizer;

use std::{collections::LinkedList, primitive};

use parser::Parser;
use tokenizer::Tokenizer;

#[derive(PartialEq, Eq)]
enum Test {
    A,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let source = std::fs::read_to_string(&args[1]).unwrap();
    let tokens = Tokenizer::tokenize(&source);
    let output = Parser::parse(&tokens);
    std::fs::write(&args[2], output).unwrap();
    // let mut a = vec![1, 2, 3, 4, 5];
    // println!("{}", a.iter().nth(0).unwrap());
    // println!("{{\n    {}\n}}", "asdad".to_string());
    // let a = Test::A;
    // let b = Test::A;
    // let c = &a;
    // let d = &b;
    // println!("{}", c == d);
}
