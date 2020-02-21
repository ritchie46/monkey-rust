use crate::code::{Instructions, OpCode, Operand};
use crate::compiler::symbol_table::SymbolTable;
use monkey::eval::object::Object;
use monkey::parser::ast::{Expression, Statement};
use std::convert::TryFrom;
use std::str::Bytes;

#[derive(Debug, Clone)]
pub struct Bytecode<'cmpl> {
    pub instructions: &'cmpl [u8],
    pub constants: &'cmpl [Object],
}

#[derive(Debug)]
struct EmittedInstruction {
    pub oc: OpCode,
    pub position: usize,
}

struct CompilationScope {
    instructions: Instructions,
    last_instruction: Option<EmittedInstruction>,
    before_last_instruction: Option<EmittedInstruction>,
}

impl CompilationScope {
    pub fn new() -> CompilationScope {
        CompilationScope {
            instructions: vec![],
            last_instruction: None,
            before_last_instruction: None,
        }
    }
}

pub struct Compiler<'cmpl> {
    scopes: Vec<CompilationScope>,
    scope_index: usize,
    constants: Vec<Object>,
    symbol_table: SymbolTable<'cmpl>,
}

impl<'cmpl> Compiler<'cmpl> {
    pub fn new() -> Compiler<'cmpl> {
        Compiler {
            scopes: vec![CompilationScope::new()],
            scope_index: 0,
            constants: vec![],
            symbol_table: SymbolTable::new(),
        }
    }

    fn current_instructions(&self) -> &[u8] {
        &self.scopes[self.scope_index].instructions
    }

    fn enter_scope(&mut self) {
        let scope = CompilationScope::new();
        self.scopes.push(scope);
        self.scope_index += 1;
    }

    fn leave_scope(&mut self) -> Vec<u8> {
        self.scope_index -= 1;
        self.scopes.pop().unwrap().instructions
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.current_instructions(),
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
            Statement::Expr(expr) => {
                self.compile_expr(expr);
                self.emit(OpCode::Pop, &[]);
            }
            Statement::Block(stmts) => {
                for stmt in stmts.iter() {
                    self.compile_stmt(stmt);
                }
            }
            Statement::Let(identifier, expr) => {
                self.compile_expr(expr);
                let index = self.symbol_table.define(identifier.to_string()).index;
                self.emit(OpCode::SetGlobal, &[index]);
            }
            Statement::Return(expr) => {
                self.compile_expr(expr);
                self.emit(OpCode::ReturnVal, &[]);
            }
            _ => panic!(format!("{:?} not catched", stmt)),
        }
    }

    fn compile_expr(&mut self, expr: &Expression) {
        match expr {
            Expression::Infix {
                left,
                operator,
                right,
            } => {
                // Reverse the constants to flip GT behavior to LT
                if operator == "<" {
                    self.compile_expr(right);
                    self.compile_expr(left);
                } else {
                    self.compile_expr(left);
                    self.compile_expr(right);
                }
                match &operator[..] {
                    "+" => {
                        self.emit(OpCode::Add, &[]);
                    }
                    "-" => {
                        self.emit(OpCode::Sub, &[]);
                    }
                    "*" => {
                        self.emit(OpCode::Mul, &[]);
                    }
                    "/" => {
                        self.emit(OpCode::Div, &[]);
                    }
                    ">" => {
                        self.emit(OpCode::GT, &[]);
                    }
                    "<" => {
                        self.emit(OpCode::GT, &[]);
                    }
                    "==" => {
                        self.emit(OpCode::Equal, &[]);
                    }
                    "!=" => {
                        self.emit(OpCode::NotEqual, &[]);
                    }
                    _ => panic!("Operand not known"),
                }
            }
            Expression::IntegerLiteral(v) => {
                let int = Object::Int(*v);
                let op = self.add_constant(int);
                self.emit(OpCode::Constant, &[op]);
            }
            Expression::Bool(v) => {
                if *v {
                    self.emit(OpCode::True, &[]);
                } else {
                    self.emit(OpCode::False, &[]);
                }
            }
            Expression::Prefix { operator, expr } => {
                self.compile_expr(expr);
                match &operator[..] {
                    "-" => {
                        self.emit(OpCode::Minus, &[]);
                    }
                    "!" => {
                        self.emit(OpCode::Bang, &[]);
                    }
                    _ => panic!(),
                }
            }
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                self.compile_expr(condition);
                // start w/ a made jump position 9999
                // jump if value on the stack is false
                let pos_jump_not_truthy = self.emit(OpCode::JumpNotTruthy, &[9999]);

                // if true stmt
                self.compile_stmt(consequence);
                if self.last_instruction_eq(OpCode::Pop) {
                    self.remove_last_pop()
                }

                // jump if consequence is executed
                let pos_jump = self.emit(OpCode::Jump, &[9999]);

                // now the length of the consequence is known we back patch the jump
                let pos_after_consequence = self.current_instructions().len();
                self.change_operand(pos_jump_not_truthy, pos_after_consequence);

                if alternative.is_none() {
                    self.emit(OpCode::Null, &[]);
                } else {
                    let alternative = alternative.as_ref().unwrap();
                    self.compile_stmt(&alternative);
                    if self.last_instruction_eq(OpCode::Pop) {
                        self.remove_last_pop()
                    }
                }
                let pos_after_alternative = self.current_instructions().len();
                self.change_operand(pos_jump, pos_after_alternative);
            }
            Expression::Identifier(ident) => {
                let opt = self.symbol_table.resolve(&ident);
                match opt {
                    None => panic!(format!("undefined variable: {}", ident)),
                    Some(smbl) => {
                        let index = smbl.index;
                        self.emit(OpCode::GetGlobal, &[index]);
                    }
                }
            }
            Expression::StringLiteral(s) => {
                let obj = Object::from(&s[..]);
                let op = self.add_constant(obj);
                self.emit(OpCode::Constant, &[op]);
            }
            Expression::ArrayLiteral(exprs) => {
                for expr in exprs.iter() {
                    self.compile_expr(expr)
                }
                self.emit(OpCode::Array, &[exprs.len()]);
            }
            Expression::FunctionLiteral { parameters, body } => {
                self.enter_scope();
                self.compile_stmt(body);
                if self.last_instruction_eq(OpCode::Pop) {
                    self.replace_last_pop_with_return()
                }
                // TODO: Maybe use only if
                else if !self.last_instruction_eq(OpCode::ReturnVal) {
                    self.emit(OpCode::Return, &[]);
                }
                let instructions = self.leave_scope();
                let compiled_fn = Object::CompiledFunction(instructions);
                let pos = self.add_constant(compiled_fn);
                self.emit(OpCode::Constant, &[pos]);
            }
            _ => panic!(),
        };
    }

    fn replace_last_pop_with_return(&mut self) {
        let last_pos = self.scopes[self.scope_index]
            .last_instruction
            .as_ref()
            .unwrap()
            .position;
        self.replace_instruction(last_pos, OpCode::ReturnVal.make(&[]));
        // get a mutable reference to the EmittedInstruction in the Option.
        self.scopes[self.scope_index]
            .last_instruction
            .as_mut()
            .unwrap()
            .oc = OpCode::ReturnVal;
    }

    /// returns memory location
    fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    fn emit(&mut self, oc: OpCode, operands: &[Operand]) -> usize {
        let ins = oc.make(operands);
        let pos = self.add_instruction(&ins);

        // Keep track of previous instructions
        let last = EmittedInstruction { oc, position: pos };
        self.scopes[self.scope_index].before_last_instruction =
            self.scopes[self.scope_index].last_instruction.replace(last);
        pos
    }

    fn add_instruction(&mut self, instructions: &[u8]) -> usize {
        // position of start new instructions
        let pos = self.current_instructions().len();
        self.scopes[self.scope_index]
            .instructions
            .extend_from_slice(instructions);
        pos
    }

    fn last_instruction_eq(&self, oc: OpCode) -> bool {
        match &self.scopes[self.scope_index].last_instruction {
            Some(emit_instr) => {
                if let oc = emit_instr.oc {
                    return true;
                }
            }
            _ => {}
        }

        false
    }

    fn remove_last_pop(&mut self) {
        let pos = match &self.scopes[self.scope_index].last_instruction {
            Some(em_ins) => em_ins.position,
            _ => panic!(),
        };

        self.scopes[self.scope_index].instructions.drain(pos..);
        let new_last = self.scopes[self.scope_index].before_last_instruction.take();
        self.scopes[self.scope_index]
            .last_instruction
            .replace(new_last.unwrap());
    }

    fn change_operand(&mut self, position: usize, operand: Operand) {
        let oc = OpCode::try_from(self.scopes[self.scope_index].instructions[position])
            .expect("Could not parse opcode");
        let new_instr = oc.make(&[operand]);
        self.replace_instruction(position, new_instr)
    }

    fn replace_instruction(&mut self, position: usize, instruction: Instructions) {
        for i in 0..instruction.len() {
            self.scopes[self.scope_index].instructions[position + i] = instruction[i]
        }
    }
}
