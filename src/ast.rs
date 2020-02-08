use crate::parser::ParseResult;
use crate::token::Token;
use std::fmt;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Statement {
    Let(String, Expression), // identifier, expr
    Return(Expression),
    Expr(Expression),
    Block(Box<Vec<Statement>>), // other statements
}

impl Statement {
    pub fn new_block(statements: Vec<Statement>) -> ParseResult<Statement> {
        Ok(Statement::Block(Box::new(statements)))
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(ident, _) => write!(f, "let {} = ", ident),
            Statement::Return(e) => write!(f, "return {}", e),
            Statement::Expr(e) => write!(f, "{}", e),
            Statement::Block(stmts) => f.write_str(&write_block(stmts)),
            _ => f.write_str("not implemented yet"),
        }
    }
}

fn write_block(stmts: &Vec<Statement>) -> String {
    let mut s = String::new();
    for b in stmts {
        s.push_str(&format!("{}", b))
    }
    s
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
        alternative: Option<Box<Statement>>,
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
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => write!(
                f,
                "if {} {{ {} }} else {{ {} }}",
                condition,
                consequence,
                write_alternative_block(alternative)
            ),
            _ => f.write_str("not impl"),
        }
    }
}

fn write_alternative_block(alt: &Option<Box<Statement>>) -> String {
    match alt {
        Some(s) => format!("{}", s),
        None => "pass".to_string(),
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

    pub fn new_if_expr(
        condition: Expression,
        consequence: Statement,
        alternative: Option<Statement>,
    ) -> ParseResult<Expression> {
        let alternative = match alternative {
            Some(stmt) => Some(Box::new(stmt)),
            None => None,
        };

        let expr = Expression::IfExpression {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative,
        };
        Ok(expr)
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
