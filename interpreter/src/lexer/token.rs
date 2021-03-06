use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum TokenType {
    Illegal,
    EOF,
    Identifier, // add, foobar, x, y
    Int,        // 123456
    Assign,     // =
    Plus,       // +
    Minus,      // -
    Comma,      // ,
    Semicolon,  // ;
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    Function,
    Let,
    Bang,     // !
    Asterix,  // *
    Slash,    // "/"
    LT,       // <
    GT,       // >
    Return,   // return
    True,     // true
    False,    // false
    If,       // if
    Else,     // else
    Equal,    // ==
    NotEqual, // !=
    Str,      // " "
    LBracket, // [
    RBracket, // ]
    Colon,    // :
    Dot,      // .
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub literal: String,
}
use TokenType::*;
lazy_static! {
    pub static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn".to_string(), Function);
        m.insert("let".to_string(), Let);
        m.insert("return".to_string(), Return);
        m.insert("true".to_string(), True);
        m.insert("false".to_string(), False);
        m.insert("if".to_string(), If);
        m.insert("else".to_string(), Else);
        m
    };
}
