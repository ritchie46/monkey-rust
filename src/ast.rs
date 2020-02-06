use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum Statement {
    Let(LetStmt),
    Return(ReturnStmt),
    Expr(ExpressionStmt),
}

#[derive(Debug)]
pub enum Expression {
    Some,
}

#[derive(Debug)]
pub struct Identifier {
    value: String,
}

impl Identifier {
    pub fn new(tkn: &Token) -> Identifier {
        Identifier {
            value: tkn.literal.clone(),
        }
    }
}

#[derive(Debug)]
pub struct LetStmt {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub value: Expression,
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub value: Expression,
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
