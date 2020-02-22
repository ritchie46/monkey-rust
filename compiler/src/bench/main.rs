#![feature(test)]
extern crate test;
use compiler::compiler::compiler::Compiler;
use compiler::utils::compile;
use compiler::vm::vm::{run_vm, VM};
use test::Bencher;

fn run_benchmark(b: &mut Bencher, input: &str) {
    let com = compile(&input).unwrap();
    let bytecode = com.bytecode();
    b.iter(|| run_vm(&bytecode));
}

#[bench]
fn bench_addition(b: &mut Bencher) {
    let input = "1 + 2";
    run_benchmark(b, &input);
}

#[bench]
fn bench_conditional(b: &mut Bencher) {
    let input = "if (1 > 2) { 10 } else { 20 }";
    run_benchmark(b, &input);
}

#[bench]
fn bench_constant_stacking(b: &mut Bencher) {
    let mut input = "".to_string();
    for i in 0..100 {
        input.push_str(&format!("{};", i))
    }
    run_benchmark(b, &input);
}

#[bench]
fn bench_array_literal(b: &mut Bencher) {
    let mut input = "[".to_string();
    for i in 0..100 {
        input.push_str(&format!("{}, ", i))
    }
    input.push_str("1]");
    run_benchmark(b, &input);
}

#[bench]
fn bench_assignment(b: &mut Bencher) {
    let input = "let one = 1; let two = one + one; one + two";
    run_benchmark(b, &input)
}
