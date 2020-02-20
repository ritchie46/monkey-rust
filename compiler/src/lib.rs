#![allow(dead_code, unused_variables, unused_imports, unused_must_use)]
#[macro_use]
extern crate lazy_static;

pub mod code;
pub mod compiler {
    pub mod compiler;
    mod symbol_table;
    mod test;
}
pub mod vm {
    mod test;
    pub mod vm;
}
mod err;
pub mod utils;

fn main() {}
