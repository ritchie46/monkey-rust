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

pub use lexer::lexer::Lexer;
pub use parser::parser::Parser;
