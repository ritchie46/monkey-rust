#[macro_use]
extern crate lazy_static;
mod ast;
mod err;
mod evaluator;
mod lexer;
mod object {
    pub mod environment;
    pub mod object;
}
mod parser;
mod repl;
mod test;
mod token;
use object::environment::Env;
use object::object::Object;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    repl::start()
}
