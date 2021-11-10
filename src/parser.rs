use core::panic;
use std::collections::BTreeMap;
use std::collections::{linked_list::Iter, LinkedList};
use std::vec;

use crate::symbol::Variable;

use super::assigner::Assigner;
use super::symbol::SymbolTable;
use super::token::Token;

pub struct Parser<'a> {
    iter: Iter<'a, Token>,
    symbol: SymbolTable,
    assigner: Assigner,
    pre_code: String,    // alloca部分，递归过程中添加代码
    block_code: String,  // 基本块部分，递归过程中添加代码
    global_code: String, // 全局变量部分，递归过程中添加代码，其实可以综合成Code类，不过这样得小重构一波
}

impl<'a> Parser<'a> {
    fn consume_token(&mut self, token: Token) {
        let var = self.iter.next().unwrap();
        if var != &token {
            panic!("syntax error, expect {:?}, but get {:?}", token, var);
        }
    }

    fn add_block_ins(&mut self, ins: String) {
        self.block_code += format!("    {}\n", ins).as_str();
    }

    fn add_pre_ins(&mut self, ins: String) {
        self.pre_code += format!("    {}\n", ins).as_str();
    }

    fn get_elem_pos(&mut self, var_name: String, pos: Vec<String>) -> String {
        let var = self.symbol.get_var(&var_name);
        let mut shape = var.shape.clone();
        let mut reg = var.reg.clone();
        if shape.len() < pos.len() {
            panic!("syntax error!");
        }
        for index in 0..pos.len() {
            let new_reg = self.assigner.new_var();
            let shape_str = Variable::get_shape_from_vec(&shape);
            self.add_block_ins(format!(
                "{} = getelementptr {}, {}* {}, i64 0, i64 {}",
                new_reg, shape_str, shape_str, reg, pos[index]
            ));
            reg = new_reg;
            shape.remove(0);
        }
        reg
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
                func_code = func_code + self.parse_func_def().as_str() + "\n";
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
            + "\n"
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
            if dimension < 0 {
                panic!("syntax error!");
            } else {
                shape.push(dimension);
            }
            self.consume_token(Token::RBracket);
        }
        // 消费赋值号
        self.consume_token(Token::Assign);
        // 逻辑处理，分为全局和局部
        if self.symbol.is_global() {
            let reg = format!("@{}", name);
            let init_val = self.parse_const_init_val(vec![], shape.clone());
            if shape.is_empty() {
                self.symbol
                    .insert_var(&name, &reg, true, &shape, atoi(&init_val, 10));
            } else {
                self.symbol.insert_var(&name, &reg, true, &shape, 0);
            }
            self.global_code += format!("{} = constant {}\n", reg, init_val).as_str();
        } else {
            let reg = self.assigner.new_pre_var();
            self.symbol.insert_var(&name, &reg, true, &shape, 0);
            let init_val = self.parse_const_init_val(vec![], shape.clone());
            self.add_pre_ins(format!("{} = alloca {}", reg, init_val));
        }
    }

    fn parse_const_init_val(&mut self, front: Vec<i32>, back: Vec<i32>) -> String {
        if self.symbol.is_global() {
            if back.is_empty() {
                format!("i32 {}", self.parse_add_exp(true).unwrap())
            } else {
                let mut res = Variable::get_shape_from_vec(&back);
                self.consume_token(Token::LBrace);
                if self.iter.clone().next().unwrap() == &Token::RBrace {
                    self.consume_token(Token::RBrace);
                    res += " zeroinitializer";
                } else {
                    res += " [";
                    let mut new_front = front.clone();
                    new_front.push(0);
                    let mut new_back = back.clone();
                    new_back.remove(0);
                    res += self
                        .parse_const_init_val(new_front.clone(), new_back.clone())
                        .as_str();
                    while *new_front.last().unwrap() < back[0] - 1 {
                        res += ", ";
                        *new_front.last_mut().unwrap() += 1;
                        match self.iter.clone().next().unwrap() {
                            Token::Comma => {
                                self.consume_token(Token::Comma);
                                res += self
                                    .parse_const_init_val(new_front.clone(), new_back.clone())
                                    .as_str();
                            }
                            _ => {
                                if new_back.is_empty() {
                                    res += "i32 0";
                                } else {
                                    res += Variable::get_shape_from_vec(&new_back).as_str();
                                    res += " zeroinitializer";
                                }
                            }
                        }
                    }
                    self.consume_token(Token::RBrace);
                    res += "]";
                }
                res
            }
        } else {
            if back.is_empty() {
                let name = self.symbol.get_current_val().name.clone();
                let mut pos: Vec<String> = vec![];
                for item in front.clone() {
                    pos.push(item.to_string());
                }
                let reg = self.get_elem_pos(name, pos);
                let val = self.parse_add_exp(true).unwrap();
                self.add_block_ins(format!("store i32 {}, i32* {}", val, reg));
            } else {
                self.consume_token(Token::LBrace);
                if self.iter.clone().next().unwrap() != &Token::RBrace {
                    let mut new_front = front.clone();
                    new_front.push(0);
                    let mut new_back = back.clone();
                    new_back.remove(0);
                    self.parse_const_init_val(new_front.clone(), new_back.clone())
                        .as_str();
                    while self.iter.clone().next().unwrap() == &Token::Comma {
                        self.consume_token(Token::Comma);
                        *new_front.last_mut().unwrap() += 1;
                        self.parse_const_init_val(new_front.clone(), new_back.clone())
                            .as_str();
                    }
                    if *new_front.last().unwrap()
                        >= self.symbol.get_current_val().shape[new_front.len() - 1]
                    {
                        panic!("syntax error!");
                    }
                }
                self.consume_token(Token::RBrace);
            }
            Variable::get_shape_from_vec(&back)
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
            if dimension < 0 {
                panic!("syntax error!");
            } else {
                shape.push(dimension);
            }
            self.consume_token(Token::RBracket);
        }
        // 根据是否有赋值号分成两类
        if self.iter.clone().next().unwrap() == &Token::Assign {
            self.consume_token(Token::Assign);
            // 进一步分为全局和局部
            if self.symbol.is_global() {
                let reg = format!("@{}", name);
                let init_val = self.parse_init_val(vec![], shape.clone());
                if shape.is_empty() {
                    self.symbol
                        .insert_var(&name, &reg, false, &shape, atoi(&init_val, 10));
                } else {
                    self.symbol.insert_var(&name, &reg, false, &shape, 0);
                }
                self.global_code += format!("{} = global {}\n", reg, init_val).as_str();
            } else {
                let reg = self.assigner.new_pre_var();
                self.symbol.insert_var(&name, &reg, false, &shape, 0);
                let init_val = self.parse_init_val(vec![], shape.clone());
                self.add_pre_ins(format!("{} = alloca {}", reg, init_val));
            }
        } else {
            // 进一步分为全局和局部
            if self.symbol.is_global() {
                let reg = format!("@{}", name);
                self.symbol.insert_var(&name, &reg, false, &shape, 0);
                let shape_str = Variable::get_shape_from_vec(&shape);
                let val_str = if shape.is_empty() {
                    "0"
                } else {
                    "zeroinitializer"
                };
                self.global_code +=
                    format!("{} = global {} {}\n", reg, shape_str, val_str).as_str();
            } else {
                let reg = self.assigner.new_pre_var();
                self.symbol.insert_var(&name, &reg, false, &shape, 0);
                let shape_str = Variable::get_shape_from_vec(&shape);
                self.add_pre_ins(format!("{} = alloca {}", reg, shape_str));
            }
        }
    }

    fn parse_init_val(&mut self, front: Vec<i32>, back: Vec<i32>) -> String {
        if self.symbol.is_global() {
            if back.is_empty() {
                format!("i32 {}", self.parse_add_exp(false).unwrap())
            } else {
                let mut res = Variable::get_shape_from_vec(&back);
                self.consume_token(Token::LBrace);
                if self.iter.clone().next().unwrap() == &Token::RBrace {
                    self.consume_token(Token::RBrace);
                    res += " zeroinitializer";
                } else {
                    res += " [";
                    let mut new_front = front.clone();
                    new_front.push(0);
                    let mut new_back = back.clone();
                    new_back.remove(0);
                    res += self
                        .parse_init_val(new_front.clone(), new_back.clone())
                        .as_str();
                    while *new_front.last().unwrap() < back[0] - 1 {
                        res += ", ";
                        *new_front.last_mut().unwrap() += 1;
                        match self.iter.clone().next().unwrap() {
                            Token::Comma => {
                                self.consume_token(Token::Comma);
                                res += self
                                    .parse_init_val(new_front.clone(), new_back.clone())
                                    .as_str();
                            }
                            _ => {
                                if new_back.is_empty() {
                                    res += "i32 0";
                                } else {
                                    res += Variable::get_shape_from_vec(&new_back).as_str();
                                    res += " zeroinitializer";
                                }
                            }
                        }
                    }
                    self.consume_token(Token::RBrace);
                    res += "]";
                }
                res
            }
        } else {
            if back.is_empty() {
                let name = self.symbol.get_current_val().name.clone();
                let mut pos: Vec<String> = vec![];
                for item in front.clone() {
                    pos.push(item.to_string());
                }
                let reg = self.get_elem_pos(name, pos);
                let val = self.parse_add_exp(false).unwrap();
                self.add_block_ins(format!("store i32 {}, i32* {}", val, reg));
            } else {
                self.consume_token(Token::LBrace);
                if self.iter.clone().next().unwrap() != &Token::RBrace {
                    let mut new_front = front.clone();
                    new_front.push(0);
                    let mut new_back = back.clone();
                    new_back.remove(0);
                    self.parse_init_val(new_front.clone(), new_back.clone())
                        .as_str();
                    while self.iter.clone().next().unwrap() == &Token::Comma {
                        self.consume_token(Token::Comma);
                        *new_front.last_mut().unwrap() += 1;
                        self.parse_init_val(new_front.clone(), new_back.clone())
                            .as_str();
                    }
                    if *new_front.last().unwrap()
                        >= self.symbol.get_current_val().shape[new_front.len() - 1]
                    {
                        panic!("syntax error!");
                    }
                }
                self.consume_token(Token::RBrace);
            }
            Variable::get_shape_from_vec(&back)
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
                self.parse_block();
            }
            Token::If => {
                // 计算条件变量
                self.consume_token(Token::If);
                self.consume_token(Token::LParen);
                let cond = self.parse_or_exp();
                self.consume_token(Token::RParen);
                // 跳转逻辑，跳转到子块
                let next_block = self.assigner.get_next_block();
                let sub_block = self.assigner.get_sub_block();
                self.add_block_ins(format!(
                    "br i1 {}, label %{}, label %{}",
                    cond, sub_block, next_block
                ));
                self.block_code += format!("{}:\n", sub_block).as_str();
                self.assigner.go_sub_block();
                // 解析子块
                self.parse_stmt();
                // 跳转逻辑，跳转到下一块，分两种情况
                self.assigner.go_parent_block();
                self.assigner.go_next_block();
                if self.iter.clone().next().unwrap() == &Token::Else {
                    let next_block = self.assigner.get_next_block();
                    self.add_block_ins(format!("br label %{}", next_block));
                } else {
                    self.add_block_ins(format!("br label %{}", next_block));
                }
                self.block_code += format!("{}:\n", next_block).as_str();
                // 解析可选的Else部分
                if self.iter.clone().next().unwrap() == &Token::Else {
                    self.consume_token(Token::Else);
                    self.parse_stmt();
                    // 跳转到Else的下一块
                    let next_block = self.assigner.get_next_block();
                    self.assigner.go_next_block();
                    self.add_block_ins(format!("br label %{}", next_block));
                    self.block_code += format!("{}:\n", next_block).as_str();
                }
            }
            Token::While => {
                // 直接进入条件跳转控制块
                let cond_block = self.assigner.get_next_block();
                self.assigner.go_next_block();
                self.assigner.new_while_block();
                self.add_block_ins(format!("br label %{}", cond_block));
                self.block_code += format!("{}:\n", cond_block).as_str();
                // 解析条件
                self.consume_token(Token::While);
                self.consume_token(Token::LParen);
                let cond = self.parse_or_exp();
                self.consume_token(Token::RParen);
                // 添加条件跳转指令
                let sub_block = self.assigner.get_sub_block();
                let next_block = self.assigner.get_next_block();
                self.add_block_ins(format!(
                    "br i1 {}, label %{}, label %{}",
                    cond, sub_block, next_block
                ));
                // 进入并解析子块
                self.block_code += format!("{}:\n", sub_block).as_str();
                self.assigner.go_sub_block();
                self.parse_stmt();
                self.assigner.go_parent_block();
                self.add_block_ins(format!("br label %{}", cond_block));
                // 进入与while同级的下一块
                self.block_code += format!("{}:\n", next_block).as_str();
            }
            Token::Break => {
                self.consume_token(Token::Break);
                // 直接进入与while同级的下一块
                let break_block = self.assigner.get_break_block();
                self.add_block_ins(format!("br label %{}", break_block));
                // 解析无效代码
                let next_block = self.assigner.get_next_block();
                self.block_code += format!("{}:\n", next_block).as_str();
            }
            Token::Continue => {
                self.consume_token(Token::Continue);
                // 直接进入条件跳转控制块
                let continue_block = self.assigner.get_continue_block();
                self.add_block_ins(format!("br label %{}", continue_block));
                // 解析无效代码
                let next_block = self.assigner.get_next_block();
                self.block_code += format!("{}:\n", next_block).as_str();
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
            panic!("const val can't be assigned!")
        }
        let mut pos: Vec<String> = vec![];
        while self.iter.clone().next().unwrap() == &Token::LBracket {
            self.consume_token(Token::LBracket);
            pos.push(self.parse_add_exp(false).unwrap());
            self.consume_token(Token::RBracket);
        }
        if pos.len() != self.symbol.get_var(name).shape.len() {
            panic!("syntax error!");
        }
        self.get_elem_pos(name.clone(), pos)
    }

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
                // 分为全局和局部
                if self.symbol.is_global() {
                    Some((-atoi(&rhs.unwrap(), 10)).to_string())
                } else {
                    let lhs = self.assigner.new_var();
                    self.add_block_ins(format!("{} = sub i32 0, {}", lhs, rhs.unwrap()));
                    Some(lhs)
                }
            }
            Token::Not => {
                // 文法中令!仅在Cond中出现
                // 比较
                let mut operand = self.parse_unary_exp(is_const).unwrap();
                let mut var = self.assigner.new_var();
                self.add_block_ins(format!("{} = icmp ne i32 {}, 0", var, operand));
                operand = var;
                // 取反
                var = self.assigner.new_var();
                self.add_block_ins(format!("{} = xor i1 {}, true", var, operand));
                operand = var;
                // 类型转换
                var = self.assigner.new_var();
                self.add_block_ins(format!("{} = zext i1 {} to i32", var, operand));
                operand = var;
                // 返回
                Some(operand)
            }
            Token::Ident(ident) => {
                // 函数调用和普通表达式计算
                if self.iter.clone().next().unwrap() == &Token::LParen {
                    // 全局域不能调用函数
                    if self.symbol.is_global() {
                        panic!("call function on global scope!");
                    }
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
                    if self.symbol.is_global() {
                        if !self.symbol.get_var(ident).is_const {
                            panic!("initializer element is not a compile-time constant!");
                        }
                        Some(self.symbol.get_var(ident).value.to_string())
                    } else {
                        let var = self.assigner.new_var();
                        let reg = self.symbol.get_var(ident).reg.clone();
                        self.add_block_ins(format!("{} = load i32, i32* {}", var, reg));
                        Some(var)
                    }

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
                    if self.symbol.is_global() {
                        operand = Some((atoi(&operand.unwrap(), 10) * atoi(&tmp, 10)).to_string());
                    } else {
                        let reg = self.assigner.new_var();
                        self.add_block_ins(format!(
                            "{} = mul i32 {}, {}",
                            reg,
                            operand.unwrap(),
                            tmp
                        ));
                        operand = Some(reg);
                    }
                }
                Some(Token::Divide) => {
                    self.consume_token(Token::Divide);
                    let tmp = self.parse_unary_exp(is_const).unwrap();
                    if self.symbol.is_global() {
                        operand = Some((atoi(&operand.unwrap(), 10) / atoi(&tmp, 10)).to_string());
                    } else {
                        let reg = self.assigner.new_var();
                        self.add_block_ins(format!(
                            "{} = sdiv i32 {}, {}",
                            reg,
                            operand.unwrap(),
                            tmp
                        ));
                        operand = Some(reg);
                    }
                }
                Some(Token::Mod) => {
                    self.consume_token(Token::Mod);
                    let tmp = self.parse_unary_exp(is_const).unwrap();
                    if self.symbol.is_global() {
                        operand = Some((atoi(&operand.unwrap(), 10) % atoi(&tmp, 10)).to_string());
                    } else {
                        let reg = self.assigner.new_var();
                        self.add_block_ins(format!(
                            "{} = srem i32 {}, {}",
                            reg,
                            operand.unwrap(),
                            tmp
                        ));
                        operand = Some(reg);
                    }
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
                    if self.symbol.is_global() {
                        operand = Some((atoi(&operand.unwrap(), 10) + atoi(&tmp, 10)).to_string());
                    } else {
                        let reg = self.assigner.new_var();
                        self.add_block_ins(format!(
                            "{} = add i32 {}, {}",
                            reg,
                            operand.unwrap(),
                            tmp
                        ));
                        operand = Some(reg);
                    }
                }
                Some(Token::Minus) => {
                    self.consume_token(Token::Minus);
                    let tmp = self.parse_mul_exp(is_const).unwrap();
                    if self.symbol.is_global() {
                        operand = Some((atoi(&operand.unwrap(), 10) - atoi(&tmp, 10)).to_string());
                    } else {
                        let reg = self.assigner.new_var();
                        self.add_block_ins(format!(
                            "{} = sub i32 {}, {}",
                            reg,
                            operand.unwrap(),
                            tmp
                        ));
                        operand = Some(reg);
                    }
                }
                _ => break,
            }
        }
        operand
    }

    fn parse_rel_exp(&mut self) -> String {
        let mut operand = self.parse_add_exp(false).unwrap();
        loop {
            match self.iter.clone().next().unwrap() {
                Token::Less => {
                    self.consume_token(Token::Less);
                    // 计算
                    let mut var = self.assigner.new_var();
                    let tmp = self.parse_rel_exp();
                    self.add_block_ins(format!("{} = icmp slt i32 {}, {}", var, operand, tmp));
                    operand = var;
                    // 类型转换
                    var = self.assigner.new_var();
                    self.add_block_ins(format!("{} = zext i1 {} to i32", var, operand));
                    operand = var;
                }
                Token::Greater => {
                    self.consume_token(Token::Greater);
                    // 计算
                    let mut var = self.assigner.new_var();
                    let tmp = self.parse_rel_exp();
                    self.add_block_ins(format!("{} = icmp sgt i32 {}, {}", var, operand, tmp));
                    operand = var;
                    // 类型转换
                    var = self.assigner.new_var();
                    self.add_block_ins(format!("{} = zext i1 {} to i32", var, operand));
                    operand = var;
                }
                Token::LessOrEqual => {
                    self.consume_token(Token::LessOrEqual);
                    // 计算
                    let mut var = self.assigner.new_var();
                    let tmp = self.parse_rel_exp();
                    self.add_block_ins(format!("{} = icmp sle i32 {}, {}", var, operand, tmp));
                    operand = var;
                    // 类型转换
                    var = self.assigner.new_var();
                    self.add_block_ins(format!("{} = zext i1 {} to i32", var, operand));
                    operand = var;
                }
                Token::GreaterOrEqual => {
                    self.consume_token(Token::GreaterOrEqual);
                    // 计算
                    let mut var = self.assigner.new_var();
                    let tmp = self.parse_rel_exp();
                    self.add_block_ins(format!("{} = icmp sge i32 {}, {}", var, operand, tmp));
                    operand = var;
                    // 类型转换
                    var = self.assigner.new_var();
                    self.add_block_ins(format!("{} = zext i1 {} to i32", var, operand));
                    operand = var;
                }
                _ => break,
            }
        }
        operand
    }

    fn parse_eq_exp(&mut self) -> String {
        let mut operand = self.parse_rel_exp();
        loop {
            match self.iter.clone().next().unwrap() {
                Token::Equal => {
                    self.consume_token(Token::Equal);
                    // 计算
                    let mut var = self.assigner.new_var();
                    let tmp = self.parse_rel_exp();
                    self.add_block_ins(format!("{} = icmp eq i32 {}, {}", var, operand, tmp));
                    operand = var;
                    // 类型转换
                    var = self.assigner.new_var();
                    self.add_block_ins(format!("{} = zext i1 {} to i32", var, operand));
                    operand = var;
                }
                Token::NotEqual => {
                    self.consume_token(Token::NotEqual);
                    // 计算
                    let mut var = self.assigner.new_var();
                    let tmp = self.parse_rel_exp();
                    self.add_block_ins(format!("{} = icmp ne i32 {}, {}", var, operand, tmp));
                    operand = var;
                    // 类型转换
                    var = self.assigner.new_var();
                    self.add_block_ins(format!("{} = zext i1 {} to i32", var, operand));
                    operand = var;
                }
                _ => break,
            }
        }
        let var = self.assigner.new_var();
        self.add_block_ins(format!("{} = icmp ne i32 {}, 0", var, operand));
        operand = var;
        operand
    }

    fn parse_and_exp(&mut self) -> String {
        let mut operand = self.parse_eq_exp();
        while self.iter.clone().next().unwrap() == &Token::And {
            self.consume_token(Token::And);
            let var = self.assigner.new_var();
            let tmp = self.parse_eq_exp();
            self.add_block_ins(format!("{} = and i1 {},{}", var, operand, tmp));
            operand = var;
        }
        operand
    }

    fn parse_or_exp(&mut self) -> String {
        let mut operand = self.parse_and_exp();
        while self.iter.clone().next().unwrap() == &Token::Or {
            self.consume_token(Token::Or);
            let var = self.assigner.new_var();
            let tmp = self.parse_and_exp();
            self.add_block_ins(format!("{} = or i1 {},{}", var, operand, tmp));
            operand = var;
        }
        operand
    }
}

fn atoi(str: &String, radix: u32) -> i32 {
    i32::from_str_radix(str.as_str(), radix).unwrap()
}
