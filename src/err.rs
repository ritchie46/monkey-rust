use std::error::Error;
use std::fmt;
use std::hash::Hasher;

#[derive(Debug)]
pub enum ParserError {
    CouldNotParse,
    IdentifierExpected,
    AssignmentExpected(String),
}

impl ParserError {
    pub fn as_str(&self) -> String {
        let s = match self {
            ParserError::CouldNotParse => "could not parse",
            ParserError::IdentifierExpected => "missing identifier",
            ParserError::AssignmentExpected(s) => {
                return format!("missing '=' after 'let {}...'", s.clone())
            }
        };
        s.to_string()
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
