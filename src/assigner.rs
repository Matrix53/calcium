use std::collections::{linked_list::Iter, LinkedList};

pub struct Assigner<'a> {
    block_pos: LinkedList<i32>,
    while_block_pos: LinkedList<Iter<'a, i32>>,
    pre_var: i32,
    var: i32,
}

impl<'a> Assigner<'a> {
    pub fn new() -> Assigner<'a> {
        Assigner {
            block_pos: LinkedList::new(),
            while_block_pos: LinkedList::new(),
            pre_var: 0,
            var: 0,
        }
    }

    pub fn new_func(&mut self) {
        self.block_pos.clear();
        self.while_block_pos.clear();
        self.pre_var = 0;
        self.var = 0;
    }

    pub fn new_pre_var(&mut self) -> String {
        self.pre_var = self.pre_var + 1;
        format!("%{}", self.pre_var)
    }

    pub fn new_var(&mut self) -> String {
        self.var = self.var + 1;
        format!("%x{}", self.var)
    }

    pub fn get_current_block(&self) -> String {
        let mut block = String::from("b");
        for item in self.block_pos {
            block += format!("_{}", item).as_str();
        }
        block
    }

    pub fn get_next_block() -> String {}

    pub fn get_sub_block() -> String {}

    pub fn get_continue_block() -> String {}

    pub fn get_break_block() -> String {}

    pub fn go_sub_block(&mut self) {
        self.block_pos.push_back(1);
    }

    pub fn go_parent_block(&mut self) {
        self.block_pos.pop_back();
    }

    pub fn go_next_block(&mut self) {
        if self.block_pos.is_empty() {
            self.block_pos.push_back(1);
        } else {
            *self.block_pos.back_mut().unwrap() += 1;
        }
    }
}
