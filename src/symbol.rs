use std::collections::{HashMap, LinkedList};

pub struct SymbolTable {
    func_table: HashMap<String, Function>,
    var_table: LinkedList<HashMap<String, Variable>>,
    current_func: String,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut table = SymbolTable {
            func_table: HashMap::new(),
            var_table: LinkedList::new(),
            current_func: String::from(""),
        };
        table.insert_func(&"getint".to_string(), true, &vec![]);
        table.insert_func(&"getch".to_string(), true, &vec![]);
        table.insert_func(&"getarray".to_string(), true, &vec![vec![0]]);
        table.insert_func(&"putint".to_string(), false, &vec![vec![]]);
        table.insert_func(&"putch".to_string(), false, &vec![vec![]]);
        table.insert_func(&"putarray".to_string(), false, &vec![vec![], vec![0]]);
        table.var_table.push_front(HashMap::new());
        table
    }

    pub fn is_global(&self) -> bool {
        self.var_table.len() == 1
    }

    pub fn get_current_func(&self) -> &Function {
        self.get_func(&self.current_func)
    }

    pub fn go_down(&mut self) {
        self.var_table.push_front(HashMap::new());
    }

    pub fn go_up(&mut self) {
        if self.var_table.is_empty() {
            panic!("bug occurs!")
        }
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

    pub fn insert_func(&mut self, func_name: &String, has_return: bool, params: &Vec<Vec<i32>>) {
        if self.func_table.contains_key(func_name) {
            panic!("redefinition of function!");
        }
        self.current_func = func_name.clone();
        self.func_table.insert(
            func_name.clone(),
            Function {
                name: func_name.clone(),
                has_return,
                params: params.clone(),
            },
        );
    }

    pub fn insert_var(
        &mut self,
        name: &String,
        reg: &String,
        is_const: bool,
        shape: &Vec<i32>,
        value: i32,
    ) {
        if self.var_table.front().unwrap().contains_key(name) {
            panic!("redefinition of variable!");
        }
        self.var_table.front_mut().unwrap().insert(
            name.clone(),
            Variable {
                reg: reg.clone(),
                is_const,
                shape: shape.clone(),
                value,
            },
        );
    }
}

pub struct Function {
    pub name: String,
    pub has_return: bool,
    pub params: Vec<Vec<i32>>,
}

impl Function {
    pub fn get_definition(&self) -> String {
        let mut params: Vec<String> = vec![];
        for item in &self.params {
            if item.is_empty() {
                params.push(format!("i32 %x{}", params.len() + 1));
            }
        }
        format!(
            "define {} @{}({}) {{\n",
            if self.has_return { "i32" } else { "void" },
            self.name,
            params.join(", ")
        )
    }

    pub fn get_call_instruction(&self, param: &Vec<String>) -> String {
        let mut params: Vec<String> = vec![];
        if self.params.len() != param.len() {
            println!("{} {}", self.params.len(), param.len());
            panic!("param count mismatches!");
        }
        for index in 0..param.len() {
            if self.params[index].is_empty() {
                params.push(format!("i32 {}", param[index]));
            }
            // TODO 对数组的处理
        }
        format!(
            "call {} @{}({})",
            if self.has_return { "i32" } else { "void" },
            self.name,
            params.join(", ")
        )
    }
}

pub struct Variable {
    pub is_const: bool,
    pub reg: String,
    pub shape: Vec<i32>,
    pub value: i32,
}

impl Variable {
    pub fn get_array_definition(&self, value: &HashMap<i32, String>) -> String {
        "".to_string()
    }
}
