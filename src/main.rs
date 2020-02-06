#[macro_use]
extern crate lazy_static;
mod ast;
mod err;
mod lexer;
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
