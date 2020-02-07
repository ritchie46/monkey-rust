use crate::parser::ParseResult;
use crate::token::Token;
use std::fmt;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Statement {
    Let(String, Expression), // identifier, expr
    Return(Expression),
    Expr(Expression),
    Block,
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(ident, _) => write!(f, "let {} = ", ident),
            Statement::Return(e) => write!(f, "return {}", e),
            Statement::Expr(e) => write!(f, "{}", e),
            _ => f.write_str("not implemented yet"),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    Prefix {
        operator: String,
        expr: Box<Expression>,
    }, // operator ('!' || '-'), expression
    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    }, // left, operator, right ex. 5 + 5
    Bool(bool),
    IfExpression {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Box<Statement>,
    },
    Some,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "{}", s),
            Expression::IntegerLiteral(int) => write!(f, "{}", int),
            Expression::Prefix { operator, expr } => write!(f, "{}{}", operator, expr),
            Expression::Infix {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
            Expression::Bool(b) => write!(f, "{}", b),
            _ => f.write_str("not impl"),
        }
    }
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
        Ok(Expression::Prefix {
            operator,
            expr: Box::new(e),
        })
    }

    pub fn new_infix_expr(
        left: Expression,
        tkn: &Token,
        right: Expression,
    ) -> ParseResult<Expression> {
        let operator = tkn.literal.to_string();
        Ok(Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
