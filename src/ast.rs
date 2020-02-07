use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::fmt;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Statement {
    Let(String, Expression), // identifier, expr
    Return(Expression),
    Expr(Expression),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(id, exp) => {
                write!(f, "Let stmt: ident: {}, expr: {:?}", id, exp)
            }
            _ => f.write_str("not implemented yet"),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    Some,
}

impl Expression {
    pub fn new_identifier(tkn: &Token) -> Expression {
        Expression::Identifier(tkn.literal.to_string())
    }

    pub fn new_integer_literal(tkn: &Token) -> Expression {
        Expression::IntegerLiteral(tkn.literal.parse::<i64>().expect(&format!(
            "Could not parse integer literal {}",
            tkn.literal
        )))
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
