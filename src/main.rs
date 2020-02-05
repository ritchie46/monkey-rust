#[macro_use]
extern crate lazy_static;

pub mod token {
    pub mod token;
}
mod lexer {
    pub mod lexer;
}
mod repl;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    repl::start()
}
