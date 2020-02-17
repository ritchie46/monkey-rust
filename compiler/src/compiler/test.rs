use super::compiler::Compiler;
use crate::code::{OpCode, Operand};
use crate::utils::{compile, parse};
use monkey::eval::object::Object;

fn make_instructions(opcodes: &[OpCode], operands: &[&[Operand]]) -> Vec<u8> {
    let mut instr = vec![];

    for (oc, op) in opcodes.iter().zip(operands) {
        instr.extend_from_slice(&oc.make(op))
    }
    instr
}

fn assert_equal_instr(input: &str, opcodes: &[OpCode], operands: &[&[Operand]]) {
    let mut com = compile(input).unwrap();
    let bc = com.bytecode();
    let instr = make_instructions(opcodes, operands);
    assert_eq!(bc.instructions, &instr);
}

#[test]
fn test_integer_arithmetic() {
    let input = "1 + 2";
    let com = compile(input).unwrap();
    let bc = com.bytecode();

    let instr = make_instructions(
        &[OpCode::Constant, OpCode::Constant, OpCode::Add, OpCode::Pop],
        &[&[0], &[1], &[], &[]],
    );

    assert_eq!(bc.constants, &[Object::Int(1), Object::Int(2)]);
    assert_eq!(bc.instructions, &instr);

    let input = "1; 2";
    assert_equal_instr(
        &input,
        &[OpCode::Constant, OpCode::Pop, OpCode::Constant, OpCode::Pop],
        &[&[0], &[], &[1], &[]],
    )
}

#[test]
fn test_boolean_exprs() {
    use OpCode::*;
    let input = "true";
    assert_equal_instr(&input, &[True, Pop], &[&[], &[]])
}
