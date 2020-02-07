use crate::ast::Statement::Expr;
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

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    Prefix(String, Box<Expression>), // operator ('!' || '-'), expression
    Infix(Box<Expression>, String, Box<Expression>), // left, operator, right ex. 5 + 5
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

    pub fn new_prefix_expr(tkn: &Token, e: Expression) -> ParseResult<Expression> {
        let operator = tkn.literal.to_string();
        Ok(Expression::Prefix(operator, Box::new(e)))
    }

    pub fn new_infix_expr(
        left: Expression,
        tkn: &Token,
        right: Expression,
    ) -> ParseResult<Expression> {
        let operator = tkn.literal.to_string();
        Ok(Expression::Infix(Box::new(left), operator, Box::new(right)))
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
