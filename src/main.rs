#[macro_use]
extern crate lazy_static;
pub mod token;
mod lexer;
mod repl;
mod ast;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    repl::start()
}
