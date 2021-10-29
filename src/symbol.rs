use std::collections::{HashMap, LinkedList};

pub struct SymbolTable {
    func_table: HashMap<String, Function>,
    var_table: LinkedList<HashMap<String, Variable>>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut table = SymbolTable {
            func_table: HashMap::new(),
            var_table: LinkedList::new(),
        };
        table.func_table.insert(
            "getint".to_string(),
            Function {
                has_return: true,
                params: vec![],
            },
        );
        table.func_table.insert(
            "getch".to_string(),
            Function {
                has_return: true,
                params: vec![],
            },
        );
        table.func_table.insert(
            "getarray".to_string(),
            Function {
                has_return: true,
                params: vec![vec![0]],
            },
        );
        table.func_table.insert(
            "putint".to_string(),
            Function {
                has_return: false,
                params: vec![vec![]],
            },
        );
        table.func_table.insert(
            "putch".to_string(),
            Function {
                has_return: false,
                params: vec![vec![]],
            },
        );
        table.func_table.insert(
            "putarray".to_string(),
            Function {
                has_return: false,
                params: vec![vec![], vec![0]],
            },
        );
        table.var_table.push_front(HashMap::new());
        table
    }

    pub fn go_down(&mut self) {
        self.var_table.push_front(HashMap::new());
    }

    pub fn go_up(&mut self) {
        self.var_table.pop_front();
    }

    pub fn get_func(&self, func_name: &String) -> &Function {
        self.func_table.get(func_name).unwrap()
    }

    pub fn get_var(&self, var_name: &String) -> &Variable {
        self.var_table
            .iter()
            .find(|table| table.contains_key(var_name))
            .unwrap()
            .get(var_name)
            .unwrap()
    }

    pub fn insert_func(&mut self, func_name: &String, func: Function) {
        if self.func_table.contains_key(func_name) {
            panic!("redefinition of function!");
        }
        self.func_table.insert(func_name.clone(), func);
    }

    pub fn insert_var(&mut self, var_name: &String, var: Variable) {
        if self.var_table.front().unwrap().contains_key(var_name) {
            panic!("redefinition of variable!");
        }
        self.var_table
            .front_mut()
            .unwrap()
            .insert(var_name.clone(), var);
    }
}

pub struct Function {
    pub has_return: bool,
    pub params: Vec<Vec<i32>>,
}

pub struct Variable {
    pub is_const: bool,
    pub name: String,
    pub shape: Vec<i32>,
}
