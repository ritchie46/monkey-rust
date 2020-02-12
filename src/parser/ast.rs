use super::parser::ParseResult;
use crate::format;
use crate::lexer::token::Token;
use std::fmt;

pub type Program = Vec<Statement>;

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
            Statement::Let(ident, e) => write!(f, "let {} = {};", ident, e),
            Statement::Return(e) => write!(f, "return {}", e),
            Statement::Expr(e) => write!(f, "{}", e),
            Statement::Block(stmts) => f.write_str(&format::fmt_block(stmts)),
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
        alternative: Option<Box<Statement>>,
    },
    FunctionLiteral {
        parameters: Box<Vec<Expression>>, // expression::identifier
        body: Box<Statement>,             // statement::block
    },
    CallExpr {
        function: Box<Expression>, // FunctionLiteral
        args: Box<Vec<Expression>>,
    },
    StringLiteral(String),
    ArrayLiteral(Box<Vec<Expression>>),
    Some, // only for debugging purposes
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
                format::fmt_alternative_block(alternative)
            ),
            Expression::FunctionLiteral {
                parameters: parameters,
                body,
            } => f.write_str(&format::fmt_function_literal(parameters, body)),
            Expression::CallExpr { function, args } => {
                f.write_str(&format::fmt_call_expr(function, args))
            }
            Expression::StringLiteral(string) => write!(f, r#""{}""#, string),
            Expression::ArrayLiteral(expressions) => {
                f.write_str(&format::fmt_array_literal(expressions))
            }
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

    pub fn new_function_literal(
        params: Vec<Expression>,
        body: Statement,
    ) -> ParseResult<Expression> {
        let expr = Expression::FunctionLiteral {
            parameters: Box::new(params),
            body: Box::new(body),
        };
        Ok(expr)
    }

    pub fn new_call_expr(
        function: Expression,
        args: Vec<Expression>,
    ) -> ParseResult<Expression> {
        let expr = Expression::CallExpr {
            function: Box::new(function),
            args: Box::new(args),
        };
        Ok(expr)
    }

    pub fn new_string_literal(tkn: &Token) -> ParseResult<Expression> {
        Ok(Expression::StringLiteral(tkn.literal.clone()))
    }

    pub fn new_array_literal(expr: Vec<Expression>) -> ParseResult<Expression> {
        Ok(Expression::ArrayLiteral(Box::new(expr)))
    }
}
