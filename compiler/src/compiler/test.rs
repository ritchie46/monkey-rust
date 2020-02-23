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

fn make_instructions_tpl(opcode_and_operand: &[(OpCode, Option<Operand>)]) -> Vec<u8> {
    let mut instr = vec![];

    for (oc, op) in opcode_and_operand.iter() {
        if op.is_none() {
            let operand = &[];
            instr.extend_from_slice(&oc.make(operand))
        } else {
            let operand = &[op.unwrap()];
            instr.extend_from_slice(&oc.make(operand))
        }
    }
    instr
}

fn assert_constants<T: Into<Object> + Clone>(input: &str, check: &[T]) {
    let com = compile(input).unwrap();
    let bc = com.bytecode();
    let check = check
        .iter()
        .cloned()
        .map(|x| x.into())
        .collect::<Vec<Object>>();
    assert_eq!(bc.constants, &check[..]);
}

fn assert_constant_literals(input: &str, check: &[Object]) {
    let com = compile(input).unwrap();
    let bc = com.bytecode();
    assert_eq!(format!("{:?}", bc.constants), format!("{:?}", check));
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
    assert_eq!(bc.instructions, &instr[..]);
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

#[test]
fn test_global_let_stmt() {
    let input = "let one = 1; let two = 2; one";
    assert_equal_instr(
        &input,
        &[Constant, SetGlobal, Constant, SetGlobal, GetGlobal, Pop],
        &[&[0], &[0], &[1], &[1], &[0], &[]],
    );
}

#[test]
fn test_strings() {
    let input = r#""monkey""#;
    assert_equal_instr(&input, &[Constant, Pop], &[&[0], &[]]);
    assert_constants(&input, &["monkey"])
}

#[test]
fn test_array_literals() {
    let input = "[]";
    assert_equal_instr(&input, &[Array, Pop], &[&[0], &[]]);

    let input = "[1, 2, 3]";
    assert_equal_instr(
        &input,
        &[Constant, Constant, Constant, Array, Pop],
        &[&[0], &[1], &[2], &[3], &[]],
    )
}

#[test]
fn test_fn_explicit_and_implicit_return() {
    let inputs = ["fn() { return 5 + 10 }", "fn() { 5 + 10 }"];

    for input in inputs.iter() {
        // The function block instruction is not in the main scope.
        assert_equal_instr(&input, &[Constant, Pop], &[&[2], &[]]);

        assert_constant_literals(
            &input,
            &[
                Object::Int(5),
                Object::Int(10),
                Object::CompiledFunction(make_instructions(
                    &[Constant, Constant, Add, ReturnVal],
                    &[&[0], &[1], &[], &[]],
                )),
            ],
        );
    }
}

#[test]
fn test_fn_no_return() {
    let input = "fn() {}";
    assert_constant_literals(
        &input,
        &[Object::CompiledFunction(make_instructions_tpl(&[(
            OpCode::Return,
            None,
        )]))],
    );
    assert_equal_instr(&input, &[Constant, Pop], &[&[0], &[]])
}

#[test]
fn test_fn_calls() {
    let input = "fn() { 24 }()";
    assert_equal_instr(&input, &[Constant, Call, Pop], &[&[1], &[], &[]]);
    let input = "let noArg = fn() { 24 };
noArg();";
    assert_equal_instr(
        &input,
        &[Constant, SetGlobal, GetGlobal, Call, Pop],
        &[&[1], &[0], &[0], &[], &[]],
    );
}

#[test]
fn test_local_assignment() {
    // only global
    let input = "let num = 55;
fn() { num }";
    assert_constant_literals(
        &input,
        &[
            Object::Int(55),
            Object::CompiledFunction(make_instructions_tpl(&[
                (GetGlobal, Some(0)),
                (ReturnVal, None),
            ])),
        ],
    );
    assert_equal_instr(
        &input,
        &[Constant, SetGlobal, Constant, Pop],
        &[&[0], &[0], &[1], &[]],
    );

    // local assignment
    let input = "fn() {
let num = 55;
num
}";
    assert_constant_literals(
        &input,
        &[
            Object::Int(55),
            Object::CompiledFunction(make_instructions_tpl(&[
                (Constant, Some(0)),
                (SetLocal, Some(0)),
                (GetLocal, Some(0)),
                (ReturnVal, None),
            ])),
        ],
    );
    assert_equal_instr(&input, &[Constant, Pop], &[&[1], &[]])
}
