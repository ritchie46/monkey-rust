use crate::eval::evaluator::eval_program;
use crate::lexer::lexer::Lexer;
use crate::eval::environment::Environment;
use crate::parser::parser::Parser;
use std::io;
use std::io::Write;

const PROMPT: &str = ">>";

pub fn start() {
    let io_in = io::stdin();
    let mut env = Environment::new();
    loop {
        // Use stdout() instead of print! macro
        // Print macro gets flushed when new line is encountered
        io::stdout().write_all([PROMPT, " "].concat().as_bytes());
        Write::flush(&mut io::stdout());

        let mut input = String::new();
        io_in.read_line(&mut input);

        let mut lex = Lexer::new(&input);
        let mut par = Parser::new(&mut lex);
        let parse_result = par.parse_program();

        match parse_result {
            Ok(program_ast) => println!("{}", eval_program(&program_ast, &mut env)),
            Err(e) => println!("{}", e),
        }
    }
}
