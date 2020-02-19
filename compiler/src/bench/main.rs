#![feature(test)]
extern crate test;
use compiler::compiler::compiler::Compiler;
use compiler::utils::compile;
use compiler::vm::vm::VM;
use test::Bencher;

#[bench]
fn bench_addition(b: &mut Bencher) {
    let input = "1 + 2";
    let com = compile(&input).unwrap();
    let bytecode = com.bytecode();
    let mut vm = VM::new(&bytecode);
    b.iter(|| vm.run());
}
