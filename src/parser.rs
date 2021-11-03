use std::collections::HashMap;
use std::collections::{linked_list::Iter, LinkedList};

use super::assigner::Assigner;
use super::symbol::SymbolTable;
use super::token::Token;

pub struct Parser<'a> {
    iter: Iter<'a, Token>,
    symbol: SymbolTable,
    assigner: Assigner,
    pre_code: String,    // alloca部分，递归过程中添加代码
    block_code: String,  // 基本块部分，递归过程中添加代码
    global_code: String, // 全局变量部分，递归过程中添加代码，其实可以综合成Code类
}

impl<'a> Parser<'a> {
    fn consume_token(&mut self, token: Token) {
        if self.iter.next().unwrap() != &token {
            panic!("syntax error!");
        }
    }

    fn add_block_ins(&mut self, ins: String) {
        self.block_code += format!("    {}\n", ins).as_str();
    }

    fn add_pre_ins(&mut self, ins: String) {
        self.pre_code += format!("    {}\n", ins).as_str();
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
            pre_code: String::new(),
            block_code: String::new(),
            global_code: String::new(),
        };
        parser.parse_comp_unit()
    }

    fn parse_comp_unit(&mut self) -> String {
        let mut func_code = String::from("");
        while self.iter.clone().next() != None {
            if self.iter.clone().nth(2).unwrap() == &Token::LParen {
                func_code += self.parse_func_def().as_str();
            } else {
                self.parse_decl();
            }
        }
        self.global_code.clone() + func_code.as_str()
    }

    fn parse_decl(&mut self) {
        match self.iter.clone().next().unwrap() {
            Token::Const => self.parse_const_decl(),
            _ => self.parse_var_decl(),
        }
    }

    fn parse_const_decl(&mut self) {
        self.consume_token(Token::Const);
        self.consume_token(Token::Int);
        self.parse_const_def();
        while self.iter.clone().next().unwrap() == &Token::Comma {
            self.consume_token(Token::Comma);
            self.parse_const_def();
        }
        self.consume_token(Token::Semicolon);
    }

    fn parse_const_def(&mut self) {
        // 标识符
        let name = match self.iter.next().unwrap() {
            Token::Ident(ident) => ident,
            _ => panic!("syntax error!"),
        };
        // 形状
        let mut shape: Vec<i32> = Vec::new();
        while self.iter.clone().next().unwrap() == &Token::LBracket {
            self.consume_token(Token::LBracket);
            let dimension = atoi(&self.parse_add_exp(), 10);
            if dimension <= 0 {
                panic!("syntax error!");
            } else {
                shape.push(dimension);
            }
            self.consume_token(Token::RBracket);
        }
        // 初始值
        self.consume_token(Token::Assign);
        let init_val = self.parse_init_val();
        // 逻辑处理
        if self.symbol.is_global() {
            // TODO 全局
        } else {
            if shape.is_empty() {
                let reg = self.assigner.new_pre_var();
                self.symbol.insert_var(
                    &name,
                    &reg,
                    true,
                    &shape,
                    atoi(&init_val.get(&0).unwrap(), 10),
                );
            } else {
                // TODO 数组
            }
        }
    }

    fn parse_var_decl(&mut self) {
        self.consume_token(Token::Int);
        self.parse_var_def();
        while self.iter.clone().next().unwrap() == &Token::Comma {
            self.consume_token(Token::Comma);
            self.parse_var_def();
        }
        self.consume_token(Token::Semicolon);
    }

    fn parse_var_def(&mut self) -> String {
        " ".to_string()
    }

    fn parse_init_val(&mut self) -> HashMap<i32, String> {
        HashMap::new()
    }

    fn parse_func_def(&mut self) -> String {
        // 声明解析
        let func_type = match self.iter.next().unwrap() {
            Token::Void => "void",
            Token::Int => "i32",
            _ => panic!("syntax error!"),
        };
        let func_name = match self.iter.next().unwrap() {
            Token::Ident(name) => name,
            _ => panic!("syntax error!"),
        };
        self.consume_token(Token::LParen);
        let func_params = match self.iter.clone().next().unwrap() {
            Token::RParen => vec![],
            _ => self.parse_func_fparams(),
        };
        self.consume_token(Token::RParen);
        // 初始化
        self.symbol
            .insert_func(func_name, func_type.eq("i32"), &func_params);
        self.assigner.reset();
        self.pre_code.clear();
        self.block_code.clear();
        // 翻译并返回
        self.parse_block();
        self.symbol.get_func(func_name).get_definition()
            + self.pre_code.as_str()
            + self.block_code.as_str()
            + "}}\n"
    }

    fn parse_func_fparams(&mut self) -> Vec<Vec<i32>> {
        vec![]
    }

    fn parse_func_fparam(&mut self) -> String {
        " ".to_string()
    }

    fn parse_block(&mut self) -> String {
        self.symbol.go_down();
        self.consume_token(Token::LBrace);
        let stmts = self.parse_stmt();
        self.consume_token(Token::RBrace);
        self.symbol.go_up();
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
}

fn atoi(str: &String, radix: u32) -> i32 {
    i32::from_str_radix(str.as_str(), radix).unwrap()
}

fn itoa(int: i32) -> String {
    int.to_string()
}
