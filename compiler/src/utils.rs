use crate::compiler::compiler::{Bytecode, Compiler};
use crate::err::CompileError;
use crate::vm::vm::VM;
use monkey::{eval::object::Object, Lexer, ParseResult, Parser, ParserError, Program};

pub fn parse(input: &str) -> Result<Program, ParserError> {
    let mut lex = Lexer::new(input);
    let mut par = Parser::new(&mut lex);
    return par.parse_program();
}

pub fn compile(input: &str) -> Result<Compiler, CompileError> {
    let ast = parse(input).expect("could not parse");
    let mut com = Compiler::new();
    com.compile_program(&ast);
    Ok(com)
}
