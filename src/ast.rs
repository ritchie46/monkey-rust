use crate::lexer::Lexer;
use crate::parser::ParseResult;
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
    pub fn new_identifier(tkn: &Token) -> ParseResult<Expression> {
        Ok(Expression::Identifier(tkn.literal.to_string()))
    }

    pub fn new_integer_literal(tkn: &Token) -> ParseResult<Expression> {
        let lit = tkn.literal.parse::<i64>()?;
        Ok(Expression::IntegerLiteral(lit))
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
