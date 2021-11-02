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
}
