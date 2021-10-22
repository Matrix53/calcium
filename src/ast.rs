pub struct CompUnit {
    pub comp_unit_items: Vec<CompUnitItem>,
}

pub enum CompUnitItem {
    Decl(Decl),
    FuncDef,
}

pub enum Decl {
    ConstDecl,
    VarDecl
}

pub struct ConstDecl{
    pub const_def:Vec<ConstDef>,
}

pub struct ConstDef{

}

pub enum ConstInitVal {
    
}

pub struct VarDecl{

}

pub struct VarDef{

}

pub enum InitVal {
    
}

pub struct FuncDef{

}

pub enum FuncType {
    
}

pub struct FuncFParams{

}

pub struct FuncFParam{

}

pub struct Block{

}

pub enum BlockItem {
    
}

pub enum Stmt {
    
}

pub struct LVal{

}

pub enum PrimaryExp {
    
}

pub enum UnaryExp {
    
}

pub enum UnaryOp {
    
}

pub struct FuncRParams{

}

pub enum MulExp {
    
}

pub enum AddExp {
    
}

pub enum RelExp {
    
}

pub enum EqExp {
    
}

pub enum LAndExp {
    
}

pub enum LOrExp {
    
}