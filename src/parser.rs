use std::collections::LinkedList;

use super::token::Token;

pub struct Parser {
    tokens: LinkedList<Token>,
}

impl Parser {
    pub fn parse(tokens: LinkedList<Token>) -> String {
        if tokens.is_empty() {
            panic!("syntax error!");
        }
        let mut parser = Parser { tokens };
        parser.parse_comp_unit()
    }

    fn parse_comp_unit(&mut self) -> String {
        "".to_string()
    }
}
