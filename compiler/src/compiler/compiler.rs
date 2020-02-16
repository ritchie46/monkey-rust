use crate::code::{Instructions, OpCode, Operand};
use monkey::eval::object::Object;
use monkey::parser::ast::{Expression, Statement};
use std::str::Bytes;

#[derive(Debug, Clone)]
pub struct Bytecode<'compiler> {
    pub instructions: &'compiler Instructions,
    pub constants: &'compiler Vec<Object>,
}

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,
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

    pub fn compile_program(&mut self, program: &[Statement]) {
        for stmt in program {
            self.compile_stmt(stmt)
        }
    }

    fn compile_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Expr(expr) => self.compile_expr(expr),
            _ => panic!(),
        }
    }

    fn compile_expr(&mut self, expr: &Expression) {
        match expr {
            Expression::Infix {
                left,
                operator,
                right,
            } => {
                self.compile_expr(left);
                self.compile_expr(right);
                match &operator[..] {
                    "+" => {
                        self.emit(OpCode::Add, &[]);
                    }
                    _ => panic!("Operand not known"),
                }
            }
            Expression::IntegerLiteral(v) => {
                let int = Object::Int(*v);
                let op = self.add_constant(int);
                self.emit(OpCode::Constant, &[op]);
            }
            _ => panic!(),
        }
    }

    /// returns memory location
    fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    fn emit(&mut self, op: OpCode, operands: &[Operand]) -> usize {
        let ins = op.make(operands);
        self.add_instruction(&ins)
    }

    fn add_instruction(&mut self, instructions: &[u8]) -> usize {
        // position of start new instructions
        let pos = instructions.len();
        self.instructions.extend_from_slice(instructions);
        pos
    }
}
