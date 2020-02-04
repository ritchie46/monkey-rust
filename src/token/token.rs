
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
