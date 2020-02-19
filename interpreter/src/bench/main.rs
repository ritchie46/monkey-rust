#![feature(test)]
extern crate test;
use test::Bencher;

use monkey::{
    eval::{environment::Environment, evaluator::eval_program},
    repl, Lexer, Parser,
};

#[bench]
fn fibonacci(bench: &mut Bencher) {
    let input = "\
    let fibonacci = fn(x) {
  if (x == 0) {
    0
  } else {
    if (x == 1) {
      return 1;
    } else {
      fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
}; fibonacci(20);
    ";
    let mut env = Environment::new();
    let mut lex = Lexer::new(&input);
    let mut par = Parser::new(&mut lex);
    let program_ast = par.parse_program().unwrap();

    bench.iter(|| eval_program(&program_ast, &mut env))
}
