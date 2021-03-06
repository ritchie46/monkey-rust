#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_must_use,
    non_shorthand_field_patterns,
    unreachable_patterns
)]
#[macro_use]
extern crate lazy_static;

mod err;
pub mod lexer {
    pub mod lexer;
    pub mod token;
}
pub mod eval {
    pub mod builtins;
    pub mod environment;
    pub mod evaluator;
    pub mod object;
}
pub mod parser {
    pub mod ast;
    pub mod parser;
}
pub mod format;
pub mod repl;
mod test;

pub use err::ParserError;
pub use lexer::lexer::Lexer;
pub use parser::{
    ast::Program,
    parser::{ParseResult, Parser},
};
