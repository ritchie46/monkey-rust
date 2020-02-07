use crate::ast::*;
use crate::err::ParserError;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::collections::{HashMap, HashSet};

pub type ParseResult<T> = Result<T, ParserError>;

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

lazy_static! {
    static ref TYPE2PREC: HashMap<TokenType, Precedence> = {
        let mut m = HashMap::new();
        m.insert(TokenType::Equal, Precedence::Equals);
        m.insert(TokenType::NotEqual, Precedence::Equals);
        m.insert(TokenType::LT, Precedence::LessGreater);
        m.insert(TokenType::GT, Precedence::LessGreater);
        m.insert(TokenType::Plus, Precedence::Sum);
        m.insert(TokenType::Minus, Precedence::Sum);
        m.insert(TokenType::Slash, Precedence::Product);
        m.insert(TokenType::Asterix, Precedence::Product);
        m
    };
    static ref INFIX_OPS: HashSet<TokenType> = {
        let mut s = HashSet::new();
        s.insert(TokenType::Plus);
        s.insert(TokenType::Minus);
        s.insert(TokenType::Slash);
        s.insert(TokenType::Asterix);
        s.insert(TokenType::Equal);
        s.insert(TokenType::NotEqual);
        s.insert(TokenType::LT);
        s.insert(TokenType::GT);
        s
    };
}

/// Get precedence of next token
fn peek_precedence(p: &Parser) -> Precedence {
    let prec = TYPE2PREC.get(&p.peek_token.type_);
    *prec.unwrap_or(&Precedence::Lowest)
}

fn current_precedence(p: &Parser) -> Precedence {
    let prec = TYPE2PREC.get(&p.current_token.type_);
    *prec.unwrap_or(&Precedence::Lowest)
}

pub struct Parser<'a> {
    lex: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}
impl<'a> Parser<'a> {
    pub fn new(lex: &'a mut Lexer<'a>) -> Parser<'a> {
        let current = lex.next_token();
        let peek = lex.next_token();

        let p = Parser {
            lex,
            current_token: current,
            peek_token: peek,
        };
        p
    }

    fn call_prefix_fn(&mut self) -> ParseResult<Expression> {
        match self.current_token.type_ {
            TokenType::Identifier => self.parse_identifier(),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Bang => self.parse_prefix_expression(),
            TokenType::Minus => self.parse_prefix_expression(),
            _ => Err(ParserError::NoPrefixParser),
        }
    }

    fn call_infix_fn(&mut self, left: Expression) -> ParseResult<Expression> {
        match self.current_token.type_ {
            TokenType::Plus => self.parse_infix_expression(left),
            TokenType::Minus => self.parse_infix_expression(left),
            TokenType::Slash => self.parse_infix_expression(left),
            TokenType::Asterix => self.parse_infix_expression(left),
            TokenType::Equal => self.parse_infix_expression(left),
            TokenType::NotEqual => self.parse_infix_expression(left),
            TokenType::LT => self.parse_infix_expression(left),
            TokenType::GT => self.parse_infix_expression(left),
            _ => Err(ParserError::ParserNotExist),
        }
    }

    fn next_token(&mut self) {
        // cannot reference because we replace peek
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut program = Program { statements: vec![] };

        while !self.current_tkn_eq(TokenType::EOF) {
            let stmt = self.parse_statement()?;

            program.statements.push(stmt);
            self.next_token();
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        let tkn = &self.current_token;

        match tkn.type_ {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    /// Check equality and if next token equal `t` consume one token.
    fn expect_and_consume_token(&mut self, t: TokenType) -> bool {
        if self.peek_token.type_ == t {
            self.next_token();
            true
        } else {
            false
        }
    }
    /// Check equality
    fn current_tkn_eq(&self, tkn: TokenType) -> bool {
        self.current_token.type_ == tkn
    }

    fn peek_tkn_eq(&self, tkn: TokenType) -> bool {
        self.peek_token.type_ == tkn
    }

    fn current_literal(&self) -> &str {
        &self.current_token.literal
    }

    fn parse_let_statement(&mut self) -> ParseResult<Statement> {
        if !self.expect_and_consume_token(TokenType::Identifier) {
            return Err(ParserError::IdentifierExpected);
        };
        let ident = self.current_literal().to_string();

        if !self.expect_and_consume_token(TokenType::Assign) {
            return Err(ParserError::AssignmentExpected(
                self.current_literal().to_string(),
            ));
        }

        // TODO: Implement Expression. We skip for now
        while !self.current_tkn_eq(TokenType::Semicolon) {
            self.next_token()
        }

        let stmt = Statement::Let(ident, Expression::Some);
        Ok(stmt)
    }

    fn parse_return_statement(&mut self) -> ParseResult<Statement> {
        self.next_token();

        // TODO: Implement Expression. We skip for now
        while !self.current_tkn_eq(TokenType::Semicolon)
            && !self.current_tkn_eq(TokenType::EOF)
        {
            self.next_token()
        }
        let stmt = Statement::Return(Expression::Some);
        Ok(stmt)
    }

    fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        let expr = self.parse_expression(Precedence::Lowest)?;

        while self.peek_tkn_eq(TokenType::Semicolon) {
            self.next_token()
        }
        let stmt = Statement::Expr(expr);
        Ok(stmt)
    }

    fn parse_expression(&mut self, prec: Precedence) -> ParseResult<Expression> {
        let mut left = self.call_prefix_fn()?;

        while !self.peek_tkn_eq(TokenType::Semicolon) && prec < peek_precedence(&self) {
            if INFIX_OPS.contains(&self.peek_token.type_) {
                self.next_token();
                left = self.call_infix_fn(left.clone())?;
            }
        }

        Ok(left)
    }

    fn parse_identifier(&mut self) -> ParseResult<Expression> {
        Expression::new_identifier(&self.current_token)
    }

    fn parse_integer_literal(&mut self) -> ParseResult<Expression> {
        Expression::new_integer_literal(&self.current_token)
    }

    fn parse_prefix_expression(&mut self) -> ParseResult<Expression> {
        let operator_tkn = self.current_token.clone();
        self.next_token();
        let right_expr = self.parse_expression(Precedence::Prefix)?;
        Expression::new_prefix_expr(&operator_tkn, right_expr)
    }

    /// Method gets called when already on infix operator
    fn parse_infix_expression(&mut self, left: Expression) -> ParseResult<Expression> {
        let prec = current_precedence(&self);
        // infix tkn {+, -, /, * ... == }
        let operator_tkn = self.current_token.clone();
        // move to next expression
        self.next_token();
        let right = self.parse_expression(prec)?;
        Expression::new_infix_expr(left, &operator_tkn, right)
    }
}
