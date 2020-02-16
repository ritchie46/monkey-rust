use super::compiler::Compiler;
use crate::code::OpCode;
use crate::utils::parse;
use monkey::eval::object::Object;

#[test]
fn test_add_constant() {
    let input = "1 + 2";
    let program = parse(input).unwrap();

    let mut comp = Compiler::new();
    comp.compile_program(&program);
    let bc = comp.bytecode();

    let mut instr = vec![];
    instr.extend_from_slice(&OpCode::Constant.make(&[0]));
    instr.extend_from_slice(&OpCode::Constant.make(&[1]));

    assert_eq!(bc.constants, &[Object::Int(1), Object::Int(2)]);
    assert_eq!(bc.instructions, &instr)
}
