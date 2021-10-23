use std::collections::{linked_list::Iter, LinkedList};

use super::token::Token;

pub struct Parser<'a> {
    iter: Iter<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn parse(tokens: &LinkedList<Token>) -> String {
        if tokens.is_empty() {
            panic!("syntax error!");
        }
        let mut parser = Parser {
            iter: tokens.iter(),
        };
        parser.parse_comp_unit()
    }

    fn parse_comp_unit(&mut self) -> String {
        "".to_string()
    }
}
