use crate::code::{Instructions, OpCode};
use std::str::Bytes;

struct Bytecode<'compiler> {
    instructions: &'compiler Instructions,
    constants: &'compiler Vec<OpCode>,
}

struct Compiler {
    instructions: Instructions,
    constants: Vec<OpCode>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            instructions: vec![],
            constants: vec![],
        }
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: &self.instructions,
            constants: &self.constants,
        }
    }
}
