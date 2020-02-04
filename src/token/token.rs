use std::collections::HashMap;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType{
    Illegal,
    EOF,
    Identifier, // add, foobar, x, y
    Int, // 123456
    Assign, // =
    Plus, // +
    Comma, // ,
    Semicolon, // ;
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    Function,
    Let
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub type_: TokenType,
    pub literal: String
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<String, TokenType> = {
    let mut m = HashMap::new();
    m.insert("fn".to_string(), TokenType::Function);
    m.insert("let".to_string(), TokenType::Let);
    m
    };
}

