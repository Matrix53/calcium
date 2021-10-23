use std::collections::LinkedList;

use super::reader::Reader;
use super::token::Token;

pub fn tokenize(source: &String) -> LinkedList<Token> {
    let mut tokens: LinkedList<Token> = LinkedList::new();
    let mut reader = Reader::new(source);
    while reader.has_next() {
        let mut chr = reader.getc();
        match chr {
            ',' => tokens.push_back(Token::Comma),
            ';' => tokens.push_back(Token::Semicolon),
            '(' => tokens.push_back(Token::LParen),
            ')' => tokens.push_back(Token::RParen),
            '[' => tokens.push_back(Token::LBracket),
            ']' => tokens.push_back(Token::RBracket),
            '{' => tokens.push_back(Token::LBrace),
            '}' => tokens.push_back(Token::RBrace),
            '+' => tokens.push_back(Token::Plus),
            '-' => tokens.push_back(Token::Minus),
            '%' => tokens.push_back(Token::Mod),
            '*' => tokens.push_back(Token::Multiply),
            '=' => {
                if reader.has_next() {
                    chr = reader.getc();
                    if chr == '=' {
                        tokens.push_back(Token::Equal);
                    } else {
                        reader.ungetc(&chr);
                        tokens.push_back(Token::Assign);
                    }
                } else {
                    tokens.push_back(Token::Assign);
                }
            }
            '!' => {
                if reader.has_next() {
                    chr = reader.getc();
                    if chr == '=' {
                        tokens.push_back(Token::NotEqual);
                    } else {
                        reader.ungetc(&chr);
                        tokens.push_back(Token::Not);
                    }
                } else {
                    tokens.push_back(Token::Not);
                }
            }
            '<' => {
                if reader.has_next() {
                    chr = reader.getc();
                    if chr == '=' {
                        tokens.push_back(Token::LessOrEqual);
                    } else {
                        reader.ungetc(&chr);
                        tokens.push_back(Token::Less);
                    }
                } else {
                    tokens.push_back(Token::Less);
                }
            }
            '>' => {
                if reader.has_next() {
                    chr = reader.getc();
                    if chr == '=' {
                        tokens.push_back(Token::GreaterOrEqual);
                    } else {
                        reader.ungetc(&chr);
                        tokens.push_back(Token::Greater);
                    }
                } else {
                    tokens.push_back(Token::Greater);
                }
            }
            '|' => {
                if reader.has_next() && reader.getc() == '|' {
                    tokens.push_back(Token::Or);
                } else {
                    panic!("syntax error!");
                }
            }
            '&' => {
                if reader.has_next() && reader.getc() == '&' {
                    tokens.push_back(Token::And);
                } else {
                    panic!("syntax error!");
                }
            }
            '/' => {
                if reader.has_next() {
                    chr = reader.getc();
                    match chr {
                        '/' => while reader.has_next() && reader.getc() != '\n' {},
                        '*' => loop {
                            while reader.has_next() && reader.getc() != '*' {}
                            if !reader.has_next() || reader.getc() == '/' {
                                break;
                            }
                        },
                        _ => {
                            reader.ungetc(&chr);
                            tokens.push_back(Token::Divide);
                        }
                    }
                } else {
                    tokens.push_back(Token::Divide);
                }
            }
            _ => {
                if chr.is_ascii_whitespace() {
                    continue;
                } else if chr.is_ascii_digit() {
                    let mut str: String = chr.to_string();
                    if chr != '0' {
                        while reader.has_next() {
                            chr = reader.getc();
                            if chr.is_ascii_digit() {
                                str.push(chr);
                            } else {
                                reader.ungetc(&chr);
                                break;
                            }
                        }
                        tokens.push_back(Token::Number(
                            i32::from_str_radix(str.as_ref(), 10).unwrap(),
                        ));
                        continue;
                    }
                    if !reader.has_next() {
                        tokens.push_back(Token::Number(0));
                        continue;
                    }
                    chr = reader.getc();
                    if chr.is_ascii_digit() {
                        str.push(chr);
                        while reader.has_next() {
                            chr = reader.getc();
                            if chr.is_ascii_digit() {
                                str.push(chr);
                            } else {
                                reader.ungetc(&chr);
                                break;
                            }
                        }
                        tokens.push_back(Token::Number(
                            i32::from_str_radix(str.as_ref(), 8).unwrap(),
                        ));
                    } else if chr == 'x' || chr == 'X' {
                        if !reader.has_next() {
                            tokens.push_back(Token::Number(0));
                            tokens.push_back(Token::Ident(chr.to_string()));
                            continue;
                        }
                        let mut tmp_chr = reader.getc();
                        if tmp_chr.is_ascii_hexdigit() {
                            str.push(tmp_chr);
                            while reader.has_next() {
                                tmp_chr = reader.getc();
                                if tmp_chr.is_ascii_hexdigit() {
                                    str.push(tmp_chr);
                                } else {
                                    reader.ungetc(&tmp_chr);
                                    break;
                                }
                            }
                            tokens.push_back(Token::Number(
                                i32::from_str_radix(str.as_ref(), 16).unwrap(),
                            ));
                        } else {
                            reader.ungetc(&tmp_chr);
                            reader.ungetc(&chr);
                            tokens.push_back(Token::Number(0));
                        }
                    } else {
                        reader.ungetc(&chr);
                        tokens.push_back(Token::Number(0));
                    }
                } else if chr.is_ascii_alphabetic() || chr == '_' {
                    let mut str: String = chr.to_string();
                    while reader.has_next() {
                        chr = reader.getc();
                        if chr.is_ascii_alphanumeric() || chr == '_' {
                            str.push(chr);
                        } else {
                            reader.ungetc(&chr);
                            break;
                        }
                    }
                    match str.as_str() {
                        "const" => tokens.push_back(Token::Const),
                        "int" => tokens.push_back(Token::Int),
                        "void" => tokens.push_back(Token::Void),
                        "if" => tokens.push_back(Token::If),
                        "else" => tokens.push_back(Token::Else),
                        "while" => tokens.push_back(Token::While),
                        "break" => tokens.push_back(Token::Break),
                        "continue" => tokens.push_back(Token::Continue),
                        "return" => tokens.push_back(Token::Return),
                        _ => tokens.push_back(Token::Ident(str)),
                    }
                } else {
                    panic!("syntax error!");
                }
            }
        }
    }
    tokens
}
