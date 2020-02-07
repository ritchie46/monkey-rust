use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum Statement {
    Let(String, Expression), // identifier, expr
    Return(Expression),
    Expr(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Some,
}

impl Expression {
    pub fn new_identifier(tkn: &Token) -> Expression {
        Expression::Identifier(tkn.literal.to_string())
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
