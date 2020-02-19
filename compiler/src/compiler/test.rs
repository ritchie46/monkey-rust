use super::compiler::Compiler;
use crate::code::{OpCode, Operand};
use crate::compiler::compiler::Bytecode;
use crate::utils::{compile, parse};
use monkey::eval::object::Object;

fn make_instructions(opcodes: &[OpCode], operands: &[&[Operand]]) -> Vec<u8> {
    let mut instr = vec![];

    for (oc, op) in opcodes.iter().zip(operands) {
        instr.extend_from_slice(&oc.make(op))
    }
    instr
}

fn assert_constants(input: &str, check: &[i64]) {
    let com = compile(input).unwrap();
    let bc = com.bytecode();
    let check = check
        .iter()
        .map(|x| Object::Int(*x))
        .collect::<Vec<Object>>();
    assert_eq!(bc.constants, &check);
}

fn assert_equal_instr(input: &str, opcodes: &[OpCode], operands: &[&[Operand]]) {
    // If fails test is not properly defined
    assert_eq!(opcodes.len(), operands.len());
    let com = compile(input).unwrap();
    let bc = com.bytecode();
    let instr = make_instructions(opcodes, operands);
    assert_eq!(bc.instructions, &instr);
}

#[test]
fn test_integer_arithmetic() {
    use OpCode::*;
    let input = "1 + 2";
    assert_constants(&input, &[1, 2]);
    assert_equal_instr(
        &input,
        &[OpCode::Constant, OpCode::Constant, OpCode::Add, OpCode::Pop],
        &[&[0], &[1], &[], &[]],
    );
    let input = "1; 2";
    assert_equal_instr(
        &input,
        &[OpCode::Constant, OpCode::Pop, OpCode::Constant, OpCode::Pop],
        &[&[0], &[], &[1], &[]],
    );
    let input = "-1";
    assert_constants(&input, &[1]);
    assert_equal_instr(&input, &[Constant, Minus, Pop], &[&[0], &[], &[]])
}

#[test]
fn test_boolean_exprs() {
    use OpCode::*;
    let input = "true";
    assert_equal_instr(&input, &[True, Pop], &[&[], &[]]);

    let input = "1 > 2";
    assert_equal_instr(
        &input,
        &[Constant, Constant, GT, Pop],
        &[&[0], &[1], &[], &[]],
    );
    assert_constants(&input, &[1, 2]);
    let input = "1 < 2";
    assert_equal_instr(
        &input,
        &[Constant, Constant, GT, Pop],
        &[&[0], &[1], &[], &[]],
    );
    assert_constants(&input, &[2, 1]); // Note the reversed constants!
    let input = "1 == 2";
    assert_equal_instr(
        &input,
        &[Constant, Constant, Equal, Pop],
        &[&[0], &[1], &[], &[]],
    );
    assert_constants(&input, &[1, 2]);
    let input = "1 != 2";
    assert_equal_instr(
        &input,
        &[Constant, Constant, NotEqual, Pop],
        &[&[0], &[1], &[], &[]],
    );
    assert_constants(&input, &[1, 2]);
    let input = "true == false";
    assert_equal_instr(&input, &[True, False, Equal, Pop], &[&[], &[], &[], &[]]);
}

#[test]
fn test_prefix() {
    use OpCode::*;
    let input = "!false";
    assert_equal_instr(&input, &[False, Bang, Pop], &[&[], &[], &[]]);
    let input = "-1";
    assert_constants(&input, &[1]);
    assert_equal_instr(&input, &[Constant, Minus, Pop], &[&[0], &[], &[]]);
}
