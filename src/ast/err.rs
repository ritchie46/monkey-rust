use std::fmt;
use std::error::Error;
use std::hash::Hasher;
use std::fmt::Write;


#[derive(Debug)]
pub enum ParserError {
    CouldNotParse
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ParserError")
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        "Could not parse token"
    }
}
