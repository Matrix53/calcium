mod parser;
mod token;
mod tokenizer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let source = std::fs::read_to_string(&args[1]).unwrap();
    let tokens = tokenizer::tokenize(&source);
    let output = parser::parse(&tokens);
    std::fs::write(&args[2], output).unwrap();
}
