mod tokenizer;
mod parser;
mod generator;
mod token;
mod ast;
fn main() {
    let args:Vec<String>=std::env::args().collect();
    let source=std::fs::read_to_string(&args[1]).unwrap();
    println!("{}",source);
}
