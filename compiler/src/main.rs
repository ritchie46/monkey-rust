#![allow(dead_code, unused_variables, unused_imports, unused_must_use)]
#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

mod code;
mod compiler {
    pub mod compiler;
    mod test;
}
mod vm {
    mod test;
    pub mod vm;
}
mod err;
mod utils;

fn main() {}
