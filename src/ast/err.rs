use std::error::Error;
use std::fmt;
use std::fmt::Write;
use std::hash::Hasher;

#[derive(Debug)]
pub enum ParserError {
    CouldNotParse,
    IdentifierExpected,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            ParserError::IdentifierExpected => "IdentifierExpected",
            _ => "ParserError",
        };
        f.write_str(s)
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        let s = match *self {
            ParserError::IdentifierExpected => "Expected a name after let",
            _ => "Could not parse token",
        };
        s
    }
}
