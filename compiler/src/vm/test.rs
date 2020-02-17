use super::vm::VM;
use crate::compiler::compiler::Compiler;
use crate::utils::{compile, parse};
use monkey::eval::object::Object;
use test::Bencher;

#[test]
fn test_addition() {
    let inout: &[(&str, i64)] = &[
        ("1 + 3", 4),
        ("3 - 2", 1),
        ("50 / 2 * 2 + 10 - 5", 55),
        ("5 * (2 + 10)", 60),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("5 * (2 + 10)", 60),
    ];

    for (input, output) in inout.iter() {
        let com = compile(&input).unwrap();
        let bytecode = com.bytecode();
        let mut vm = VM::new(&bytecode);
        vm.run();
        assert_eq!(vm.last_popped(), &Object::Int(*output));
    }
}

#[bench]
fn bench_addition(b: &mut Bencher) {
    let input = "1 + 2";
    let com = compile(&input).unwrap();
    let bytecode = com.bytecode();
    let mut vm = VM::new(&bytecode);
    b.iter(|| vm.run());
}
