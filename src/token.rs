pub enum Token {
    Ident(String),
    Number(i32),
    Const,
    Int,
    Void,
    If,
    Else,
    While,
    Break,
    Continue,
    Return,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Equal,
    NotEqual,
    Assign,
    Plus,
    Minus,
    Not,
    Multiply,
    Divide,
    Mod,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    And,
    Or,
}