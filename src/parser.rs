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
        String::from(
            "declare i32 @getint()\n\
        declare i32 @getch()\n\
        declare i32 @getarray(i32*)\n\
        declare void @putch(i32)\n\
        declare void @putint(i32)\n\
        declare void @putarray(i32, i32*)\n\n",
        ) + self.global_code.clone().as_str()
            + func_code.as_str()
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
            let dimension = atoi(&self.parse_add_exp(true).unwrap(), 10);
            if dimension <= 0 {
                panic!("syntax error!");
            } else {
                shape.push(dimension);
            }
            self.consume_token(Token::RBracket);
        }
        // 初始值
        self.consume_token(Token::Assign);
        let init_val = self.parse_const_init_val();
        // 逻辑处理
        if self.symbol.is_global() {
            // TODO 全局
        } else {
            if shape.is_empty() {
                let reg = self.assigner.new_pre_var();
                self.symbol.insert_var(&name, &reg, true, &shape, 0);
                self.add_pre_ins(format!("{} = alloca i32", reg));
                self.add_block_ins(format!(
                    "store i32 {}, i32* {}",
                    init_val.get(&0).unwrap(),
                    reg
                ));
            } else {
                // TODO 数组
            }
        }
    }

    fn parse_const_init_val(&mut self) -> HashMap<i32, String> {
        let mut res: HashMap<i32, String> = HashMap::new();
        if self.iter.clone().next().unwrap() != &Token::LBrace {
            res.insert(0, self.parse_add_exp(true).unwrap());
            res
        } else {
            self.consume_token(Token::LBrace);
            if self.iter.clone().next().unwrap() != &Token::RBrace {
                let mut son = self.parse_init_val();
                // TODO 数组下标转换
                while self.iter.clone().next().unwrap() == &Token::Comma {
                    self.consume_token(Token::Comma);
                    son = self.parse_init_val();
                    // TODO 数组下标转换
                }
            }
            self.consume_token(Token::RBrace);
            res
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

    fn parse_var_def(&mut self) {
        // 标识符
        let name = match self.iter.next().unwrap() {
            Token::Ident(ident) => ident,
            _ => panic!("syntax error!"),
        };
        // 形状
        let mut shape: Vec<i32> = Vec::new();
        while self.iter.clone().next().unwrap() == &Token::LBracket {
            self.consume_token(Token::LBracket);
            let dimension = atoi(&self.parse_add_exp(true).unwrap(), 10);
            if dimension <= 0 {
                panic!("syntax error!");
            } else {
                shape.push(dimension);
            }
            self.consume_token(Token::RBracket);
        }
        // 初始值
        let init_val = match self.iter.clone().next().unwrap() {
            Token::Assign => {
                self.consume_token(Token::Assign);
                self.parse_init_val()
            }
            _ => HashMap::new(),
        };
        // 逻辑处理
        if self.symbol.is_global() {
            // TODO 全局
        } else {
            if shape.is_empty() {
                let reg = self.assigner.new_pre_var();
                self.symbol.insert_var(&name, &reg, false, &shape, 0);
                self.add_pre_ins(format!("{} = alloca i32", reg));
                if !init_val.is_empty() {
                    self.add_block_ins(format!(
                        "store i32 {}, i32* {}",
                        init_val.get(&0).unwrap(),
                        reg
                    ));
                }
            } else {
                // TODO 数组
            }
        }
    }

    fn parse_init_val(&mut self) -> HashMap<i32, String> {
        let mut res: HashMap<i32, String> = HashMap::new();
        if self.iter.clone().next().unwrap() != &Token::LBrace {
            res.insert(0, self.parse_add_exp(false).unwrap());
            res
        } else {
            self.consume_token(Token::LBrace);
            if self.iter.clone().next().unwrap() != &Token::RBrace {
                let mut son = self.parse_init_val();
                // TODO 数组下标转换
                while self.iter.clone().next().unwrap() == &Token::Comma {
                    self.consume_token(Token::Comma);
                    son = self.parse_init_val();
                    // TODO 数组下标转换
                }
            }
            self.consume_token(Token::RBrace);
            res
        }
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
        self.block_code.push_str("b_1:\n");
        self.assigner.go_next_block();
        self.parse_block();
        self.add_pre_ins("br label %b_1".to_string());
        self.symbol.get_func(func_name).get_definition()
            + self.pre_code.as_str()
            + self.block_code.as_str()
            + "}\n"
    }

    fn parse_func_fparams(&mut self) -> Vec<Vec<i32>> {
        vec![]
    }

    fn parse_func_fparam(&mut self) -> String {
        " ".to_string()
    }

    fn parse_block(&mut self) {
        self.symbol.go_down();
        self.consume_token(Token::LBrace);
        while self.iter.clone().next().unwrap() != &Token::RBrace {
            self.parse_block_item();
        }
        self.consume_token(Token::RBrace);
        self.symbol.go_up();
    }

    fn parse_block_item(&mut self) {
        let next = self.iter.clone().next().unwrap();
        if next == &Token::Const || next == &Token::Int {
            self.parse_decl();
        } else {
            self.parse_stmt();
        }
    }

    fn parse_stmt(&mut self) {
        match self.iter.clone().next().unwrap() {
            Token::Return => {
                self.consume_token(Token::Return);
                if self.iter.clone().next().unwrap() == &Token::Semicolon {
                    self.consume_token(Token::Semicolon);
                    if self.symbol.get_current_func().has_return {
                        panic!("return value mismatches!")
                    }
                    self.add_block_ins("ret void".to_string());
                } else {
                    let ret_val = self.parse_add_exp(false).unwrap();
                    if !self.symbol.get_current_func().has_return {
                        panic!("return value mismatches!")
                    }
                    self.add_block_ins(format!("ret i32 {}", ret_val));
                }
            }
            Token::Ident(ident) => {
                match self
                    .iter
                    .clone()
                    .find(|&item| item == &Token::Assign || item == &Token::Semicolon)
                    .unwrap()
                {
                    Token::Assign => {
                        let lhs = self.parse_lval();
                        self.consume_token(Token::Assign);
                        let rhs = self.parse_add_exp(false);
                        self.consume_token(Token::Semicolon);
                        self.add_block_ins(format!("store i32 {}, i32* {}", rhs.unwrap(), lhs));
                    }
                    Token::Semicolon => {
                        self.parse_add_exp(false);
                        self.consume_token(Token::Semicolon);
                    }
                    _ => panic!("bug occurs, unreachable code!"),
                }
            }
            Token::LBrace => {
                panic!("lab hasn't finished!")
            }
            Token::If => {
                panic!("lab hasn't finished!")
            }
            Token::While => {
                panic!("lab hasn't finished!")
            }
            Token::Break => {
                panic!("lab hasn't finished!")
            }
            Token::Continue => {
                panic!("lab hasn't finished!")
            }
            _ => {
                if self.iter.clone().next().unwrap() != &Token::Semicolon {
                    self.parse_add_exp(false);
                }
                self.consume_token(Token::Semicolon);
            }
        }
    }

    fn parse_lval(&mut self) -> String {
        let name = match self.iter.next().unwrap() {
            Token::Ident(ident) => ident,
            _ => panic!("syntax error!"),
        };
        if self.symbol.get_var(name).is_const {
            panic!("lval can't be assigned!")
        }
        if self.iter.clone().next().unwrap() == &Token::LBracket {
            // TODO 数组下标解析
            self.consume_token(Token::LBracket);
            self.consume_token(Token::RBracket);
            panic!("syntax error!")
        }
        self.symbol.get_var(name).reg.clone()
    }

    // TODO 剩余的逻辑
    fn parse_unary_exp(&mut self, is_const: bool) -> Option<String> {
        match self.iter.next().unwrap() {
            Token::Number(num) => Some(num.to_string()),
            Token::LParen => {
                let ans = self.parse_add_exp(is_const);
                self.consume_token(Token::RParen);
                ans
            }
            Token::Plus => self.parse_unary_exp(is_const),
            Token::Minus => {
                let rhs = self.parse_unary_exp(is_const);
                let lhs = self.assigner.new_var();
                self.add_block_ins(format!("{} = sub i32 0, {}", lhs, rhs.unwrap()));
                Some(lhs)
            }
            Token::Ident(ident) => {
                if self.iter.clone().next().unwrap() == &Token::LParen {
                    // 收集参数
                    self.consume_token(Token::LParen);
                    let params = match self.iter.clone().next().unwrap() {
                        Token::RParen => Vec::new(),
                        _ => self.parse_func_rparams(),
                    };
                    self.consume_token(Token::RParen);
                    // 调用并返回
                    if self.symbol.get_func(ident).has_return {
                        let reg = self.assigner.new_var();
                        self.add_block_ins(format!(
                            "{} = {}",
                            reg,
                            self.symbol.get_func(ident).get_call_instruction(&params)
                        ));
                        Some(reg)
                    } else {
                        self.add_block_ins(
                            self.symbol.get_func(ident).get_call_instruction(&params),
                        );
                        None
                    }
                    // TODO 数组参数的处理
                } else {
                    if is_const && !self.symbol.get_var(ident).is_const {
                        panic!("var occurs in const expression!");
                    }
                    let var = self.assigner.new_var();
                    let reg = self.symbol.get_var(ident).reg.clone();
                    self.add_block_ins(format!("{} = load i32, i32* {}", var, reg));
                    Some(var)
                    // TODO 数组下标的逻辑
                }
            }
            _ => panic!("syntax error!"),
        }
    }

    // TODO 参数类型检查, 利用get_current_func
    fn parse_func_rparams(&mut self) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        res.push(self.parse_add_exp(false).unwrap());
        while self.iter.clone().next().unwrap() == &Token::Comma {
            self.consume_token(Token::Comma);
            res.push(self.parse_add_exp(false).unwrap())
        }
        res
    }

    fn parse_mul_exp(&mut self, is_const: bool) -> Option<String> {
        let mut operand = self.parse_unary_exp(is_const);
        loop {
            match self.iter.clone().next() {
                Some(Token::Multiply) => {
                    self.consume_token(Token::Multiply);
                    let tmp = self.parse_unary_exp(is_const).unwrap();
                    let reg = self.assigner.new_var();
                    self.add_block_ins(format!("{} = mul i32 {}, {}", reg, operand.unwrap(), tmp));
                    operand = Some(reg);
                }
                Some(Token::Divide) => {
                    self.consume_token(Token::Divide);
                    let tmp = self.parse_unary_exp(is_const).unwrap();
                    let reg = self.assigner.new_var();
                    self.add_block_ins(format!("{} = sdiv i32 {}, {}", reg, operand.unwrap(), tmp));
                    operand = Some(reg);
                }
                Some(Token::Mod) => {
                    self.consume_token(Token::Mod);
                    let tmp = self.parse_unary_exp(is_const).unwrap();
                    let reg = self.assigner.new_var();
                    self.add_block_ins(format!("{} = srem i32 {}, {}", reg, operand.unwrap(), tmp));
                    operand = Some(reg);
                }
                _ => break,
            }
        }
        operand
    }

    fn parse_add_exp(&mut self, is_const: bool) -> Option<String> {
        let mut operand = self.parse_mul_exp(is_const);
        loop {
            match self.iter.clone().next() {
                Some(Token::Plus) => {
                    self.consume_token(Token::Plus);
                    let tmp = self.parse_mul_exp(is_const).unwrap();
                    let reg = self.assigner.new_var();
                    self.add_block_ins(format!("{} = add i32 {}, {}", reg, operand.unwrap(), tmp));
                    operand = Some(reg);
                }
                Some(Token::Minus) => {
                    self.consume_token(Token::Minus);
                    let tmp = self.parse_mul_exp(is_const).unwrap();
                    let reg = self.assigner.new_var();
                    self.add_block_ins(format!("{} = sub i32 {}, {}", reg, operand.unwrap(), tmp));
                    operand = Some(reg);
                }
                _ => break,
            }
        }
        operand
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
