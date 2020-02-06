use crate::ast::err::ParserError;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::string::ParseError;

#[derive(Debug)]
enum Statement {
    Let(LetStatement),
}

#[derive(Debug)]
enum Expression {
    Some,
}

type ParseResult<T> = Result<T, ParserError>;

trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
struct Identifier {
    value: String,
}

impl Identifier {
    fn new(tkn: &Token) -> Identifier {
        Identifier {
            value: tkn.literal.clone(),
        }
    }
}

#[derive(Debug)]
struct LetStatement {
    name: Identifier,
    value: Expression,
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
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

    fn next_token(&mut self) {
        // cannot reference because we replace peek
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut program = Program { statements: vec![] };

        while self.current_token.type_ != TokenType::EOF {
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
            _ => Err(ParserError::CouldNotParse),
        }
    }

    fn expect_and_consume_token(&mut self, t: TokenType) -> bool {
        if self.peek_token.type_ == t {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn parse_let_statement(&mut self) -> ParseResult<Statement> {
        if !self.expect_and_consume_token(TokenType::Identifier) {
            return Err(ParserError::IdentifierExpected);
        };
        let ident = Identifier::new(&self.current_token);

        if !self.expect_and_consume_token(TokenType::Assign) {
            return Err(ParserError::AssignmentExpected);
        }

        // TODO: Implement Expression. We skip for now
        while self.current_token.type_ != TokenType::Semicolon {
            self.next_token()
        }

        Ok(Statement::Let(LetStatement {
            value: Expression::Some,
            name: ident,
        }))
    }
}
