use crate::err::ParserError;
use crate::err::ParserError::Expected;
use crate::eval::object::Object;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::*;
use std::collections::{HashMap, HashSet};

pub type ParseResult<T> = Result<T, ParserError>;

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Method,
    Prefix,
    Call,
    Index,
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
        m.insert(TokenType::LParen, Precedence::Call);
        m.insert(TokenType::LBracket, Precedence::Index);
        m.insert(TokenType::Dot, Precedence::Method);
        m
    };
}

/// Get precedence of next token
fn peek_precedence(p: &Parser) -> Precedence {
    let prec = TYPE2PREC.get(&p.peek_token.type_);
    *prec.unwrap_or(&Precedence::Lowest)
}

fn current_precedence(p: &Parser) -> Precedence {
    let prec = TYPE2PREC.get(&p.current_type());
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
        match self.current_type() {
            TokenType::Identifier => self.parse_identifier(),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Bang => self.parse_prefix_expr(),
            TokenType::Minus => self.parse_prefix_expr(),
            TokenType::True => self.parse_bool(),
            TokenType::False => self.parse_bool(),
            TokenType::LParen => self.parse_grouped_expr(),
            TokenType::If => self.parse_if_expr(),
            TokenType::Function => self.parse_function_literal(),
            TokenType::Str => self.parse_string_literal(),
            TokenType::LBracket => self.parse_array_literal(),
            TokenType::LBrace => self.parse_hash_literal(),
            // Try to parse it and let evaluator define errors.
            _ => self.parse_prefix_expr(),
        }
    }

    fn call_infix_fn(&mut self, left: Expression) -> ParseResult<Expression> {
        match self.current_type() {
            TokenType::Plus => self.parse_infix_expr(left),
            TokenType::Minus => self.parse_infix_expr(left),
            TokenType::Slash => self.parse_infix_expr(left),
            TokenType::Asterix => self.parse_infix_expr(left),
            TokenType::Equal => self.parse_infix_expr(left),
            TokenType::NotEqual => self.parse_infix_expr(left),
            TokenType::LT => self.parse_infix_expr(left),
            TokenType::GT => self.parse_infix_expr(left),
            TokenType::LParen => self.parse_call_expr(left), // left is fn
            TokenType::LBracket => self.parse_index_expr(left),
            TokenType::Dot => self.parse_method(left),
            // Try to parse it and let evaluator define errors.
            _ => self.parse_infix_expr(left),
        }
    }

    fn next_token(&mut self) {
        // cannot reference because we replace peek
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    fn current_type(&self) -> TokenType {
        self.current_token.type_
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut program: Program = vec![];

        while !self.current_tkn_eq(TokenType::EOF) {
            let stmt = self.parse_statement()?;

            program.push(stmt);
            self.next_token();
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        let tkn = &self.current_token;

        match tkn.type_ {
            TokenType::Let => self.parse_let_stmnt(),
            TokenType::Return => self.parse_return_stmnt(),
            _ => self.parse_expression_stmnt(),
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

    fn parse_let_stmnt(&mut self) -> ParseResult<Statement> {
        if !self.expect_and_consume_token(TokenType::Identifier) {
            return Err(ParserError::IdentifierExpected);
        };
        let ident = self.current_literal().to_string();

        if !self.expect_and_consume_token(TokenType::Assign) {
            return Err(ParserError::AssignmentExpected(
                self.current_literal().to_string(),
            ));
        }
        self.next_token();

        let value = self.parse_expr(Precedence::Lowest)?;

        let stmt = Statement::Let(ident, value);
        self.expect_and_consume_token(TokenType::Semicolon);

        Ok(stmt)
    }

    fn parse_return_stmnt(&mut self) -> ParseResult<Statement> {
        self.next_token();

        let return_val = self.parse_expr(Precedence::Lowest)?;
        self.expect_and_consume_token(TokenType::Semicolon);
        let stmt = Statement::Return(return_val);
        Ok(stmt)
    }

    /// The heart of the parser
    /// Read chapter 2.8 for an explanation.
    fn parse_expression_stmnt(&mut self) -> ParseResult<Statement> {
        let expr = self.parse_expr(Precedence::Lowest)?;

        while self.peek_tkn_eq(TokenType::Semicolon) {
            self.next_token()
        }
        let stmt = Statement::Expr(expr);
        Ok(stmt)
    }

    fn parse_expr(&mut self, prec: Precedence) -> ParseResult<Expression> {
        let mut left = self.call_prefix_fn()?;

        while !self.peek_tkn_eq(TokenType::Semicolon) && prec < peek_precedence(&self) {
            if TYPE2PREC.contains_key(&self.peek_token.type_) {
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

    fn parse_prefix_expr(&mut self) -> ParseResult<Expression> {
        let operator_tkn = self.current_token.clone();
        self.next_token();
        let right_expr = self.parse_expr(Precedence::Prefix)?;
        Expression::new_prefix_expr(&operator_tkn, right_expr)
    }

    /// Method gets called when already on infix operator
    fn parse_infix_expr(&mut self, left: Expression) -> ParseResult<Expression> {
        let prec = current_precedence(&self);
        // infix tkn {+, -, /, * ... == }
        let operator_tkn = self.current_token.clone();
        // move to next expression
        self.next_token();
        let right = self.parse_expr(prec)?;
        Expression::new_infix_expr(left, &operator_tkn, right)
    }

    fn parse_bool(&mut self) -> ParseResult<Expression> {
        Ok(Expression::Bool(self.current_tkn_eq(TokenType::True)))
    }

    fn parse_grouped_expr(&mut self) -> ParseResult<Expression> {
        self.next_token();
        let expr = self.parse_expr(Precedence::Lowest)?;

        if !self.expect_and_consume_token(TokenType::RParen) {
            return Err(ParserError::CouldNotParse(
                "missing right paren ')'".to_string(),
            ));
        }
        Ok(expr)
    }

    fn parse_if_expr(&mut self) -> ParseResult<Expression> {
        if !self.expect_and_consume_token(TokenType::LParen) {
            return Err(ParserError::CouldNotParse(
                "missing left paren '('".to_string(),
            ));
        }
        self.next_token();
        let condition = self.parse_expr(Precedence::Lowest)?;

        if !self.expect_and_consume_token(TokenType::RParen) {
            return Err(ParserError::CouldNotParse(
                "missing right paren ')'".to_string(),
            ));
        }

        if !self.expect_and_consume_token(TokenType::LBrace) {
            return Err(ParserError::CouldNotParse(
                "missing left brace '{'".to_string(),
            ));
        }

        let consequence = self.parse_block_stmt()?;
        let mut alternative = None;

        if self.peek_tkn_eq(TokenType::Else) {
            self.next_token();

            if !self.expect_and_consume_token(TokenType::LBrace) {
                return Err(ParserError::CouldNotParse(
                    "missing left brace '{'".to_string(),
                ));
            }
            alternative = Some(self.parse_block_stmt()?)
        }
        Expression::new_if_expr(condition, consequence, alternative)
    }

    fn parse_block_stmt(&mut self) -> ParseResult<Statement> {
        let mut stmts = vec![];

        self.next_token();

        while !self.current_tkn_eq(TokenType::RBrace)
            && !self.current_tkn_eq(TokenType::EOF)
        {
            let stmt = self.parse_statement()?;
            stmts.push(stmt);
            self.next_token();
        }
        Statement::new_block(stmts)
    }

    fn parse_function_literal(&mut self) -> ParseResult<Expression> {
        if !self.expect_and_consume_token(TokenType::LParen) {
            return Err(ParserError::CouldNotParse(
                "missing left paren '('".to_string(),
            ));
        }
        let params = self.parse_function_params()?;

        if !self.expect_and_consume_token(TokenType::LBrace) {
            return Err(ParserError::CouldNotParse(
                "missing left brace '{'".to_string(),
            ));
        }
        let body = self.parse_block_stmt()?;

        Expression::new_function_literal(params, body)
    }

    fn parse_function_params(&mut self) -> ParseResult<Vec<Expression>> {
        self.parse_comma_separated_expressions(TokenType::RParen)
    }

    fn parse_array_literal(&mut self) -> ParseResult<Expression> {
        let expr = self.parse_comma_separated_expressions(TokenType::RBracket)?;
        Expression::new_array_literal(expr)
    }

    /// Parse comma separated expressions ended by TokenType
    fn parse_comma_separated_expressions(
        &mut self,
        end_tkn: TokenType,
    ) -> ParseResult<Vec<Expression>> {
        let mut expressions: Vec<Expression> = vec![];

        if self.peek_tkn_eq(end_tkn) {
            self.next_token();
            // return empty expr vector
            return Ok(expressions);
        }
        self.next_token();

        let expr = self.parse_expr(Precedence::Lowest)?;

        expressions.push(expr);

        while self.peek_tkn_eq(TokenType::Comma) {
            // skip comma
            self.next_token();
            self.next_token();
            let expr = self.parse_expr(Precedence::Lowest)?;
            expressions.push(expr);
        }

        if !self.expect_and_consume_token(end_tkn) {
            return Err(ParserError::CouldNotParse(format!(
                "missing ending token: {:?}",
                end_tkn
            )));
        }
        Ok(expressions)
    }

    fn parse_call_expr(&mut self, function: Expression) -> ParseResult<Expression> {
        let args = self.parse_call_args()?;
        Expression::new_call_expr(function, args)
    }

    fn parse_call_args(&mut self) -> ParseResult<Vec<Expression>> {
        let mut args: Vec<Expression> = vec![];

        if self.expect_and_consume_token(TokenType::RParen) {
            return Ok(args);
        }
        self.next_token();
        args.push(self.parse_expr(Precedence::Lowest)?);

        while self.peek_tkn_eq(TokenType::Comma) {
            // skip comma
            self.next_token();
            self.next_token();
            args.push(self.parse_expr(Precedence::Lowest)?);
        }

        if !self.expect_and_consume_token(TokenType::RParen) {
            return Err(ParserError::CouldNotParse(
                "missing right paren in function call ')'".to_string(),
            ));
        }
        Ok(args)
    }

    fn parse_string_literal(&mut self) -> ParseResult<Expression> {
        Expression::new_string_literal(&self.current_token)
    }

    fn parse_index_expr(&mut self, left: Expression) -> ParseResult<Expression> {
        self.next_token();
        let index = self.parse_expr(Precedence::Lowest)?;
        if !self.expect_and_consume_token(TokenType::RBracket) {
            return Err(ParserError::Expected("]".to_string()));
        }
        Expression::new_index_expr(left, index)
    }

    fn parse_method(&mut self, left: Expression) -> ParseResult<Expression> {
        self.next_token(); // dot
        let ident = self.parse_identifier()?;

        if !self.peek_tkn_eq(TokenType::LParen) {
            return Expression::new_method(left, ident, vec![]);
        };

        self.next_token(); // (
        let args = self.parse_call_args()?;
        Expression::new_method(left, ident, args)
    }

    fn parse_hash_literal(&mut self) -> ParseResult<Expression> {
        let mut keys = vec![];
        let mut values = vec![];

        while !self.peek_tkn_eq(TokenType::RBrace) {
            self.next_token();

            let key = self.parse_expr(Precedence::Lowest)?;
            if !self.expect_and_consume_token(TokenType::Colon) {
                return Err(ParserError::Expected(":".to_string()));
            };

            self.next_token();
            let value = self.parse_expr(Precedence::Lowest)?;
            keys.push(key);
            values.push(value);

            if !self.peek_tkn_eq(TokenType::RBrace)
                && !self.expect_and_consume_token(TokenType::Comma)
            {
                return Err(ParserError::Expected(",".to_string()));
            }
        }
        if !self.expect_and_consume_token(TokenType::RBrace) {
            return Err(ParserError::Expected("}".to_string()));
        }
        Expression::new_hash_literal(keys, values)
    }
}
