use std::collections::{linked_list::Iter, LinkedList};

use super::assigner::Assigner;
use super::symbol::SymbolTable;
use super::token::Token;

pub struct Parser<'a> {
    iter: Iter<'a, Token>,
    symbol: SymbolTable,
    assigner: Assigner,
}

impl<'a> Parser<'a> {
    fn consume_token(&mut self, token: Token) {
        if self.iter.next().unwrap() != &token {
            panic!("syntax error!");
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse(tokens: &LinkedList<Token>) -> String {
        if tokens.is_empty() {
            panic!("syntax error!");
        }
        let mut parser = Parser {
            iter: tokens.iter(),
            symbol: SymbolTable::new(),
            assigner: Assigner::new(),
        };
        parser.parse_comp_unit()
    }

    fn parse_comp_unit(&mut self) -> String {
        match self.iter.clone().nth(2).unwrap() {
            Token::LParen => self.parse_func_def(),
            _ => self.parse_decl(),
        }
    }

    fn parse_decl(&mut self) -> String {
        panic!("syntax error!");
        " ".to_string()
    }

    fn parse_const_decl(&mut self) -> String {
        " ".to_string()
    }

    fn parse_const_def(&mut self) -> String {
        " ".to_string()
    }

    fn parse_const_init_val(&mut self) -> String {
        " ".to_string()
    }

    fn parse_var_decl(&mut self) -> String {
        " ".to_string()
    }

    fn parse_var_def(&mut self) -> String {
        " ".to_string()
    }

    fn parse_init_val(&mut self) -> String {
        " ".to_string()
    }

    fn parse_func_def(&mut self) -> String {
        let func_type = match self.iter.next().unwrap() {
            Token::Void => "void",
            Token::Int => "i32",
            _ => panic!("syntax error!"),
        };
        let func_name = match self.iter.next().unwrap() {
            Token::Ident(name) => name,
            _ => panic!("syntax error!"),
        };
        // 当前lab的特判
        if func_name != "main" {
            panic!("syntax error!");
        }
        self.consume_token(Token::LParen);
        let func_params = match self.iter.clone().next().unwrap() {
            Token::RParen => "".to_string(),
            _ => self.parse_func_fparams(),
        };
        self.consume_token(Token::RParen);
        let block = self.parse_block();
        format!(
            "define {} @{}({}) {{\n{}\n}}\n",
            func_type, func_name, func_params, block
        )
    }

    fn parse_func_fparams(&mut self) -> String {
        " ".to_string()
    }

    fn parse_func_fparam(&mut self) -> String {
        " ".to_string()
    }

    fn parse_block(&mut self) -> String {
        self.consume_token(Token::LBrace);
        let stmts = self.parse_stmt();
        self.consume_token(Token::RBrace);
        stmts
    }

    fn parse_block_item(&mut self) -> String {
        " ".to_string()
    }

    fn parse_stmt(&mut self) -> String {
        self.consume_token(Token::Return);
        let number = self.parse_add_exp();
        self.consume_token(Token::Semicolon);
        format!("    ret i32 {}", number)
    }

    fn parse_lval(&mut self) -> String {
        " ".to_string()
    }

    fn parse_unary_exp(&mut self) -> String {
        match self.iter.next().unwrap() {
            Token::Number(num) => num.to_string(),
            Token::LParen => {
                let ans = self.parse_add_exp();
                self.consume_token(Token::RParen);
                ans
            }
            Token::Plus => self.parse_unary_exp(),
            Token::Minus => {
                let ans = i32::from_str_radix(self.parse_unary_exp().as_str(), 10).unwrap();
                (-ans).to_string()
            }
            _ => panic!("syntax error!"),
        }
    }

    fn parse_func_rparams(&mut self) -> String {
        " ".to_string()
    }

    fn parse_mul_exp(&mut self) -> String {
        let mut operand = i32::from_str_radix(self.parse_unary_exp().as_str(), 10).unwrap();
        loop {
            match self.iter.clone().next() {
                Some(Token::Multiply) => {
                    self.iter.next();
                    let tmp = i32::from_str_radix(self.parse_unary_exp().as_str(), 10).unwrap();
                    operand *= tmp;
                }
                Some(Token::Divide) => {
                    self.iter.next();
                    let tmp = i32::from_str_radix(self.parse_unary_exp().as_str(), 10).unwrap();
                    operand /= tmp;
                }
                Some(Token::Mod) => {
                    self.iter.next();
                    let tmp = i32::from_str_radix(self.parse_unary_exp().as_str(), 10).unwrap();
                    operand %= tmp;
                }
                _ => break,
            }
        }
        format!("{}", operand)
    }

    fn parse_add_exp(&mut self) -> String {
        let mut operand = i32::from_str_radix(self.parse_mul_exp().as_str(), 10).unwrap();
        loop {
            match self.iter.clone().next() {
                Some(Token::Plus) => {
                    self.iter.next();
                    let tmp = i32::from_str_radix(self.parse_mul_exp().as_str(), 10).unwrap();
                    operand += tmp;
                }
                Some(Token::Minus) => {
                    self.iter.next();
                    let tmp = i32::from_str_radix(self.parse_mul_exp().as_str(), 10).unwrap();
                    operand -= tmp;
                }
                _ => break,
            }
        }
        format!("{}", operand)
    }

    fn parse_rel_exp(&mut self) -> String {
        " ".to_string()
    }

    fn parse_eq_exp(&mut self) -> String {
        " ".to_string()
    }

    fn parse_and_exp(&mut self) -> String {
        " ".to_string()
    }

    fn parse_or_exp(&mut self) -> String {
        " ".to_string()
    }

    fn parse_const_exp(&mut self) -> String {
        " ".to_string()
    }
}
