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
            _ => {}
        }
    }
    tokens
}
