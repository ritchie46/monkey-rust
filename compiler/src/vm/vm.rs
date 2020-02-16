use crate::code::{read_be_u16, OpCode};
use crate::compiler::compiler::Bytecode;
use crate::err::VMError;
use monkey::eval::object::Object;

const STACKSIZE: usize = 2048;

pub struct VM<'cmpl> {
    constants: &'cmpl [Object],
    instructions: &'cmpl [u8],

    pub stack: Vec<Object>,
    sp: usize, // Points to the next free registry on the stack
}

impl VM<'_> {
    pub fn new<'cmpl>(bytecode: &'cmpl Bytecode) -> VM<'cmpl> {
        VM {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack: Vec::with_capacity(STACKSIZE),
            sp: 0,
        }
    }
    fn stack_top(&self) -> Option<&Object> {
        if self.sp == 0 {
            None
        } else {
            Some(&self.stack[self.sp - 1])
        }
    }

    fn pop(&mut self) -> Option<Object> {
        self.stack.pop()
    }

    fn push(&mut self, o: Object) -> Result<(), VMError> {
        if self.sp >= STACKSIZE {
            return Err(VMError::StackOverflow);
        }

        self.stack.push(o);
        self.sp += 1;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        let mut i = 0;
        while i < self.instructions.len() {
            let op = OpCode::from(self.instructions[i]);
            match op {
                OpCode::Constant => {
                    let const_index = read_be_u16(&self.instructions[i + 1..]) as usize;
                    i += 2;
                    let r = self.push(self.constants[const_index].clone())?;
                }
            }
            i += 1;
        }
        Ok(())
    }
}
