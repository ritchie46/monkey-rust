#[macro_use]
extern crate lazy_static;
mod err;
mod lexer {
    pub mod lexer;
    pub mod token;
}
mod eval {
    pub mod builtins;
    pub mod environment;
    pub mod evaluator;
    pub mod object;
}
mod parser {
    pub mod ast;
    pub mod parser;
}
mod repl;
mod test;
use eval::environment::Env;
use eval::object::Object;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    repl::start()
}
