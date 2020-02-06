#[macro_use]
extern crate lazy_static;
mod lexer;
mod repl;
pub mod token;
mod ast {
    mod ast;
    mod test;
}

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    repl::start()
}
