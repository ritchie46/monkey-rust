use crate::token::TokenType;
use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    CouldNotParse(String),
    IdentifierExpected,
    AssignmentExpected(String),
    NoParserFor(TokenType),
}

impl ParserError {
    pub fn as_str(&self) -> String {
        match self {
            ParserError::CouldNotParse(s) => format!("could not parse: {}", s),
            ParserError::IdentifierExpected => "missing identifier".to_string(),
            ParserError::AssignmentExpected(s) => {
                format!("missing '=' after 'let {}...'", s)
            }
            _ => "".to_string(),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_type = match *self {
            ParserError::IdentifierExpected => "IdentifierExpected",
            ParserError::AssignmentExpected(_) => "AssignmentExpected",
            _ => "ParserError",
        };
        let s = format!("{}: {}", err_type, self.as_str());
        f.write_str(&s)
    }
}

impl From<std::num::ParseIntError> for ParserError {
    fn from(error: std::num::ParseIntError) -> Self {
        ParserError::CouldNotParse("Integer".to_string())
    }
}
