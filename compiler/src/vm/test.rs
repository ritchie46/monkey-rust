use super::vm::VM;
use crate::compiler::compiler::Compiler;
use crate::utils::{compile, parse};
use monkey::eval::object::Object;

fn run_vm(input: &str) -> Object {
    let com = compile(&input).unwrap();
    let bytecode = com.bytecode();
    let mut vm = VM::new(&bytecode);
    vm.run();
    vm.last_popped().clone()
}

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

#[test]
fn test_bools() {
    let inout = &[("true", true), ("false", false)];
    for (input, output) in inout.iter() {
        let com = compile(&input).unwrap();
        let bytecode = com.bytecode();
        let mut vm = VM::new(&bytecode);
        vm.run();
        assert_eq!(vm.last_popped(), &Object::Bool(*output));
    }
}

#[test]
fn test_cmp() {
    let inout = &[
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
    ];
    for (input, output) in inout {
        assert_eq!(run_vm(&input), Object::Bool(*output));
    }
}

#[test]
fn test_prefix() {
    let inout = &[
        ("!true", false),
        ("!!true", true),
        ("!0", true),
        ("!5", false),
        ("!!5", true),
    ];
    for (input, output) in inout {
        assert_eq!(run_vm(&input), Object::Bool(*output));
    }
    let inout = &[("-5", -5), ("--5", 5)];
    for (input, output) in inout {
        assert_eq!(run_vm(&input), Object::Int(*output));
    }
}

#[test]
fn test_conditional() {
    let inout = &[
        ("if (true) { 10 }", 10),
        ("if (true) { 10 } else { 20 }", 10),
        ("if (false) { 10 } else { 20 }", 20),
        ("if (1) { 10 } else { 20 }", 10),
        ("if (1 > 2) { 10 } else { 20 }", 20),
    ];
    for (input, output) in inout {
        assert_eq!(run_vm(&input), Object::Int(*output));
    }

    use Object::{Bool, Null};
    let inout = &[
        ("if (1 > 2) { 10 }", Null),
        ("if (false) { 10 }", Null),
        ("!(if (false) { 5; })", Bool(true)), // tests that we don't get null from conditional
    ];
    for (input, output) in inout {
        assert_eq!(run_vm(&input), *output);
    }
}

#[test]
fn test_assignment() {
    let inout = &[
        ("let one = 1; one", 1),
        ("let one = 1; let two = 2; one + two", 3),
        ("let one = 1; let two = one + one; one + two", 3),
    ];
    for (input, output) in inout {
        assert_eq!(run_vm(&input), Object::Int(*output));
    }
}

#[test]
fn test_string_literals() {
    let inout = &[
        (r#""monkey""#, "monkey"),
        (r#""mon" + "key""#, "monkey"),
        (r#""mon" + "key" + "banana""#, "monkeybanana"),
    ];
    for (input, output) in inout {
        assert_eq!(run_vm(&input), Object::from(*output));
    }
}

#[test]
fn test_array_literals() {
    let inout = &[
        ("[]", Object::new_array(vec![])),
        (
            "[1, 2]",
            Object::new_array(vec![Object::Int(1), Object::Int(2)]),
        ),
    ];
    for (input, output) in inout {
        assert_eq!(format!("{}", run_vm(&input)), format!("{}", *output));
    }
}

#[test]
fn test_fn_call() {
    let inout = &[
        ("let fivePlusTen = fn() { 5 + 10; }; fivePlusTen();", 15),
        (
            "let one = fn() { 1; };
let two = fn() { 2; };
one() + two()",
            3,
        ),
        (
            "let a = fn() { 1 };
let b = fn() { a() + 1 };
let c = fn() { b() + 1 };
c();",
            3,
        ),
    ];
    for (input, output) in inout {
        assert_eq!(run_vm(&input), Object::from(*output));
    }
}
