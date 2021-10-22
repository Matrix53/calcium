mod ast;
mod generator;
mod parser;
mod token;
mod tokenizer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let source = std::fs::read_to_string(&args[1]).unwrap();
    let tokens = tokenizer::tokenize(&source);
    let ast = parser::parse(&tokens);
    let output = generator::generate(&ast);
    std::fs::write(&args[2], output).unwrap();
}
