#[macro_use]
extern crate lazy_static;
mod ast;
mod err;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod test;
mod token;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    repl::start()
}
