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
        "123".to_string()
    }

    fn parse_decl(&mut self) -> String {
        "123".to_string()
    }

    fn parse_const_decl(&mut self) -> String {
        "123".to_string()
    }

    fn parse_const_def(&mut self) -> String {
        "123".to_string()
    }

    fn parse_const_init_val(&mut self) -> String {
        "123".to_string()
    }

    fn parse_var_decl(&mut self) -> String {
        "123".to_string()
    }

    fn parse_var_def(&mut self) -> String {
        "123".to_string()
    }

    fn parse_init_val(&mut self) -> String {
        "123".to_string()
    }

    fn parse_func_def(&mut self) -> String {
        "123".to_string()
    }

    fn parse_func_fparams(&mut self) -> String {
        "123".to_string()
    }

    fn parse_func_fparam(&mut self) -> String {
        "123".to_string()
    }

    fn parse_block(&mut self) -> String {
        "123".to_string()
    }

    fn parse_block_item(&mut self) -> String {
        "123".to_string()
    }

    fn parse_stmt(&mut self) -> String {
        "123".to_string()
    }

    fn parse_lval(&mut self) -> String {
        "123".to_string()
    }

    fn parse_unary_exp(&mut self) -> String {
        "123".to_string()
    }

    fn parse_unary_op(&mut self) -> String {
        "123".to_string()
    }

    fn parse_func_rparams(&mut self) -> String {
        "123".to_string()
    }

    fn parse_mul_exp(&mut self) -> String {
        "123".to_string()
    }

    fn parse_add_exp(&mut self) -> String {
        "123".to_string()
    }

    fn parse_rel_exp(&mut self) -> String {
        "123".to_string()
    }

    fn parse_eq_exp(&mut self) -> String {
        "123".to_string()
    }

    fn parse_and_exp(&mut self) -> String {
        "123".to_string()
    }

    fn parse_or_exp(&mut self) -> String {
        "123".to_string()
    }

    fn parse_const_exp(&mut self) -> String {
        "123".to_string()
    }
}
