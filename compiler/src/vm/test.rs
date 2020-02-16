use super::vm::VM;
use crate::compiler::compiler::Compiler;
use crate::utils::{compile, parse};
use monkey::eval::object::Object;

#[test]
fn test_addition() {
    let input = "1 + 2";
    let com = compile(&input).unwrap();
    let bytecode = com.bytecode();
    let mut vm = VM::new(&bytecode);
    vm.run();
    assert_eq!(vm.stack, [Object::Int(3)]);
}
