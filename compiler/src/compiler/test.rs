use super::compiler::Compiler;
use crate::code::{read_operands, OpCode, Operand};
use crate::compiler::compiler::Bytecode;
use crate::utils::{compile, parse};
use monkey::eval::object::Object;
use std::convert::TryFrom;

use OpCode::*;

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

fn write_human_readable(instr: &[u8]) {
    let mut i = 0;
    while i < instr.len() {
        let start_ptr = i;
        let b = instr[i];
        let oc = OpCode::try_from(b).unwrap();
        let operands = read_operands(oc.definition(), &instr[i..]).0;
        for width in oc.definition() {
            i += *width
        }
        i += 1;
        println!("{:04}\t{:?}\t\t{:?}", start_ptr, oc, operands)
    }
}

fn assert_equal_instr(input: &str, opcodes: &[OpCode], operands: &[&[Operand]]) {
    // If fails test is not properly defined
    assert_eq!(opcodes.len(), operands.len());
    let com = compile(input).unwrap();
    let bc = com.bytecode();
    let instr = make_instructions(opcodes, operands);
    println!("WANT:");
    write_human_readable(&instr);
    println!("\nGOT:");
    write_human_readable(&bc.instructions);
    assert_eq!(bc.instructions, &instr);
}

#[test]
fn test_integer_arithmetic() {
    let input = "1 + 2";
    assert_constants(&input, &[1, 2]);
    assert_equal_instr(
        &input,
        &[Constant, Constant, Add, Pop],
        &[&[0], &[1], &[], &[]],
    );
    let input = "1; 2";
    assert_equal_instr(
        &input,
        &[Constant, Pop, Constant, Pop],
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
    let input = "!false";
    assert_equal_instr(&input, &[False, Bang, Pop], &[&[], &[], &[]]);
    let input = "-1";
    assert_constants(&input, &[1]);
    assert_equal_instr(&input, &[Constant, Minus, Pop], &[&[0], &[], &[]]);
}

#[test]
fn test_conditional_if() {
    let input = "if (true) { 10 }; 3333;";
    assert_equal_instr(
        &input,
        &[
            True,
            JumpNotTruthy,
            Constant,
            Jump,
            Null,
            Pop,
            Constant,
            Pop,
        ],
        &[&[], &[10], &[0], &[11], &[], &[], &[1], &[]],
    );
    assert_constants(&input, &[10, 3333]);
}

#[test]
fn test_condition_if_else() {
    let input = "if (true) { 10 } else {20}; 3333;";
    assert_equal_instr(
        &input,
        // the if else expr has only one pop at the end. As only on stmt is being executed
        //  we can pop after one of those has been executed
        &[
            True,
            JumpNotTruthy,
            Constant,
            Jump,
            Constant,
            Pop,
            Constant,
            Pop,
        ],
        &[&[], &[10], &[0], &[13], &[1], &[], &[2], &[]],
    );
}
