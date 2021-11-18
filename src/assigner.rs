pub struct Assigner {
    block_pos: Vec<i32>,
    while_block_pos: Vec<usize>,
    pre_var: i32,
    var: i32,
}

impl Assigner {
    pub fn new() -> Assigner {
        Assigner {
            block_pos: Vec::new(),
            while_block_pos: Vec::new(),
            pre_var: 0,
            var: 0,
        }
    }

    pub fn reset(&mut self) {
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

    pub fn new_while_block(&mut self) {
        self.while_block_pos.push(self.block_pos.len());
    }

    pub fn get_current_block(&self) -> String {
        let mut block = String::from("b");
        for item in self.block_pos.iter() {
            block += format!("_{}", item).as_str();
        }
        block
    }

    pub fn get_next_block(&mut self) -> String {
        *self.block_pos.last_mut().unwrap() += 1;
        let block = self.get_current_block();
        *self.block_pos.last_mut().unwrap() -= 1;
        block
    }

    pub fn get_sub_block(&mut self) -> String {
        self.get_current_block() + "_1"
    }

    pub fn get_continue_block(&mut self) -> String {
        let mut block = String::from("b");
        for item in 0..*self.while_block_pos.last().unwrap() {
            block += format!("_{}", self.block_pos[item]).as_str();
        }
        block
    }

    pub fn get_break_block(&mut self) -> String {
        self.block_pos[*self.while_block_pos.last().unwrap() - 1] += 1;
        let mut block = String::from("b");
        for item in 0..*self.while_block_pos.last().unwrap() {
            block += format!("_{}", self.block_pos[item]).as_str();
        }
        self.block_pos[*self.while_block_pos.last().unwrap() - 1] -= 1;
        block
    }

    pub fn go_sub_block(&mut self) {
        self.block_pos.push(1);
    }

    pub fn go_parent_block(&mut self) {
        self.block_pos.pop();
        self.while_block_pos.sort();
        while !self.while_block_pos.is_empty()&&self.while_block_pos.last().unwrap()>=&self.block_pos.len(){
            self.while_block_pos.pop();
        }
    }

    pub fn go_next_block(&mut self) {
        if self.block_pos.is_empty() {
            self.block_pos.push(1);
        } else {
            *self.block_pos.last_mut().unwrap() += 1;
        }
    }
}
