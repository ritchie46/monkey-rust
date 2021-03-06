use crate::err::ParserError;
use crate::eval::{environment::Environment, object::Object};
use crate::lexer::lexer::Lexer;
use crate::parser::ast::*;
use crate::parser::parser::*;

fn parse_program(input: &str) -> Result<Program, ParserError> {
    let mut lex = Lexer::new(&input);
    let mut par = Parser::new(&mut lex);
    par.parse_program()
}

#[cfg(test)]
mod parser_test {
    use super::*;

    #[test]
    fn test_parser_errors() {
        let input = "let x;";
        let parsed = parse_program(&input).unwrap_err();
        match parsed {
            ParserError::AssignmentExpected(_) => assert!(true),
            _ => assert!(false),
        }
        let input = "let =";
        let parsed = parse_program(&input).unwrap_err();
        match parsed {
            ParserError::IdentifierExpected => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_precedence() {
        assert!(Precedence::Lowest < Precedence::Equals);
        assert!(Precedence::Prefix == Precedence::Prefix)
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let parsed = parse_program(&input);
        assert_eq!(
            Statement::Expr(Expression::Identifier("foobar".to_string())),
            parsed.unwrap()[0]
        );
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let parsed = parse_program(&input);
        assert_eq!(
            Statement::Expr(Expression::IntegerLiteral(5 as i64)),
            parsed.unwrap()[0]
        );
    }

    #[test]
    fn test_prefix_expression() {
        let input = "-5;";
        let parsed = parse_program(&input);
        assert_eq!(
            Statement::Expr(Expression::Prefix {
                operator: "-".to_string(),
                expr: Box::new(Expression::IntegerLiteral(5 as i64))
            }),
            parsed.unwrap()[0]
        );
    }

    fn test_operator_precedence_parsing(inputs: &[&str], outputs: &[&str]) {
        for (input, output) in inputs.iter().zip(outputs) {
            let parsed = parse_program(input).unwrap();
            assert_eq!(format!("{}", parsed[0]), *output)
        }
    }

    #[test]
    fn test_infix_expression() {
        let input = "-5 == 10;";
        let parsed = parse_program(&input).unwrap();
        let inputs = [
            "a + b * c + d / e - f",
            "a != 10;",
            "c > 6;",
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
        ];
        let outputs = [
            "(((a + (b * c)) + (d / e)) - f)",
            "(a != 10)",
            "(c > 6)",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ];
        test_operator_precedence_parsing(&inputs, &outputs)
    }

    #[test]
    fn test_bool_expression() {
        let inputs = ["true", "false", "3 > 5 == false"];
        let outputs = ["true", "false", "((3 > 5) == false)"];
        test_operator_precedence_parsing(&inputs, &outputs)
    }

    #[test]
    fn test_grouped_expression() {
        let inputs = ["1 + (2 + 3) + 4"];
        let outputs = ["((1 + (2 + 3)) + 4)"];
        test_operator_precedence_parsing(&inputs, &outputs)
    }

    #[test]
    fn test_if_expr() {
        let input = "if (x < y) { x }";
        let parsed = parse_program(&input);
        assert_eq!(
            "if (x < y) { x } else { pass }",
            format!("{}", parsed.unwrap()[0])
        );

        let input = "if (x < y) { x } else { y }";
        let parsed = parse_program(&input);
        assert_eq!(
            "if (x < y) { x } else { y }",
            format!("{}", parsed.unwrap()[0])
        );
    }

    #[test]
    fn test_function_literal() {
        let input = "fn(a, b) { a * b }";
        let parsed = parse_program(&input);
        assert_eq!("fn(a, b) { (a * b) }", format!("{}", parsed.unwrap()[0]));
        let input = "fn(x, y, z) {}";
        let parsed = parse_program(&input);
        assert_eq!("fn(x, y, z) {  }", format!("{}", parsed.unwrap()[0]));
    }

    #[test]
    fn test_call_expr() {
        let inputs = [
            "add(1, 2 * 3, 4 + 5);",
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a + b + c * d / f + g)",
        ];
        let outputs = [
            "add(1, (2 * 3), (4 + 5))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            "add((((a + b) + ((c * d) / f)) + g))",
        ];
        test_operator_precedence_parsing(&inputs, &outputs)
    }

    #[test]
    fn test_return_stmt() {
        let input = "return add(9 * 6 + 3)";
        let parsed = parse_program(&input);
        assert_eq!(
            "return add(((9 * 6) + 3))",
            format!("{}", parsed.unwrap()[0])
        );
    }

    #[test]
    fn test_let_stmt() {
        let input = "let foo = call(9 /3);";
        let parsed = parse_program(&input);
        assert_eq!(
            "let foo = call((9 / 3));",
            format!("{}", parsed.unwrap()[0])
        );
    }

    #[test]
    fn test_str_lit() {
        let input = r#" "foo" "#;
        let parsed = parse_program(&input);
        assert_eq!(r#""foo""#, format!("{}", parsed.unwrap()[0]));
    }
}

#[cfg(test)]
mod eval_test {
    use super::*;
    use crate::eval::evaluator::eval_program;

    fn evaluated(input: &str) -> Object {
        let parsed = parse_program(input);
        let mut env = Environment::new();
        let ev = eval_program(&parsed.unwrap(), &mut env);
        ev.clone()
    }

    #[test]
    fn test_int_eval() {
        let input = "5";
        let ev = evaluated(&input);
        assert_eq!(Object::Int(5), ev)
    }

    #[test]
    fn test_bool_eval() {
        for (input, output) in ["true", "false"].iter().zip(&[true, false]) {
            let ev = evaluated(&input);
            assert_eq!(Object::Bool(*output), ev)
        }
    }

    #[test]
    fn test_eval() {
        for (input, output) in ["!true", "!!false"].iter().zip(&[false, false]) {
            let ev = evaluated(&input);
            assert_eq!(Object::Bool(*output), ev)
        }
        for (input, output) in ["-5", "--5"].iter().zip(&[-5, 5]) {
            let ev = evaluated(&input);
            assert_eq!(Object::Int(*output), ev)
        }
        let inputs = ["-593", "5 + 2 * 10", "(5 + 10 * 2 + 15 / 3) * 2 + -10"];
        let outputs = [-593, 25, 50];

        for (input, output) in inputs.iter().zip(&outputs) {
            let ev = evaluated(&input);
            assert_eq!(Object::Int(*output), ev)
        }

        let inputs = [
            "true == true",
            "(1 < 2) == true",
            "(1 > 2) == false",
            "(1 > 2) == true",
        ];
        let outputs = [true, true, true, false];

        for (input, output) in inputs.iter().zip(&outputs) {
            let ev = evaluated(&input);
            assert_eq!(Object::Bool(*output), ev)
        }

        let inputs = [
            "if (true) { 4 }",
            "if ( 5 < 4 ) { 1 } else { 2 }",
            "if ( false ) { 1 } else {2}",
        ];
        let outputs = [4, 2, 2];
        for (input, output) in inputs.iter().zip(&outputs) {
            let ev = evaluated(&input);
            assert_eq!(Object::Int(*output), ev)
        }

        // no alternative should return null
        let out = evaluated(&"if (false) {1};");
        assert_eq!(out, Object::Null);

        let inputs = [
            "return 10;",
            "return 10;9",
            "return 2 * 5; 9;",
            "9; return 2 * 5; 9;",
            // early return
            "if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }
                return 1;
             }",
        ];
        let outputs = [10, 10, 10, 10, 10];
        for (input, output) in inputs.iter().zip(&outputs) {
            let ev = evaluated(&input);
            assert_eq!(ev, Object::Int(*output))
        }

        let inputs = ["if (10 > 1) { true + false; }", "-true", "5 + true; 6"];
        let outputs = [
            "Error: unknown operator: bool + bool",
            "Error: unknown operator: -bool",
            "Error: type mismatch: int + bool",
        ];
        for (input, output) in inputs.iter().zip(&outputs) {
            let ev = evaluated(&input);
            assert_eq!(ev, Object::Error(output.to_string()))
        }

        let inputs = ["let a = 15; a", "let b = 30 / 3; 10;"];
        let outputs = [15, 10];
        for (input, output) in inputs.iter().zip(&outputs) {
            let ev = evaluated(&input);
            assert_eq!(ev, Object::Int(*output))
        }
    }

    #[test]
    fn test_function_eval() {
        let inputs = [
            "let add = fn(x, y) { x + y }; add(2, 2);",
            "let identity = fn(x) { x; }; identity(5);",
            "let identity = fn(x) { return x; }; identity(5);",
            "let double = fn(x) { x * 2; }; double(5);",
            "let add = fn(x, y) { x + y; }; add(5, 5)",
            "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
            "fn(x) { x; }(5)",
        ];
        let outputs = [4, 5, 5, 10, 10, 20, 5];
        for (input, output) in inputs.iter().zip(&outputs) {
            let ev = evaluated(&input);
            assert_eq!(ev, Object::Int(*output))
        }
    }

    #[test]
    fn test_str_lit_eval() {
        let inputs = [
            r#""foo""#,
            r#""foo bar ham""#,
            r#""foo" + "ham"#,
            r#""foo"+"ham"#,
        ];
        let outputs = ["foo", "foo bar ham", "fooham", "fooham"];
        for (input, output) in inputs.iter().zip(&outputs) {
            let ev = evaluated(&input);
            assert_eq!(ev, Object::String(output.to_string()))
        }
    }

    #[test]
    fn test_builtin() {
        let inputs = [r#"len("bar")"#, r#"len("foo" + "bar")"#];
        let outputs = [3, 6];
        for (input, output) in inputs.iter().zip(&outputs) {
            let ev = evaluated(&input);
            assert_eq!(ev, Object::Int(*output))
        }
    }

    #[test]
    fn test_array() {
        let inputs = ["[1, 2, 3, 4]", r#"["foo", 1, false]"#];
        for (input, output) in inputs.iter().zip(&inputs) {
            let parsed = &parse_program(&input).unwrap()[0];
            assert_eq!(&format!("{}", parsed), input);

            let ev = evaluated(&input);
            assert_eq!(&format!("{}", ev), input);
        }

        let input = "let a = [1, 2]; a[0]";
        let parsed = &parse_program(input).unwrap();
        assert_eq!("a[0]", format!("{}", parsed[1]));

        let inputs = [
            "[1, 2, 3][0]",
            "[1, 2, 3][1]",
            "[1, 2, 3][2]",
            "let i = 0; [1][i];",
            "[1, 2, 3][-1]",
            "let a = [1, 2, 3]; len(a)",
        ];
        let outputs = [1, 2, 3, 1, 3, 3];
        for (input, output) in inputs.iter().zip(&outputs) {
            let ev = evaluated(&input);
            assert_eq!(ev, Object::Int(*output))
        }
    }

    #[test]
    fn test_hash() {
        let inputs = [
            r#"{1: "one", 2: "two"}"#,
            r#"{"one": 0 + 1, "two": 10 - 8, "three": 15 / 5}"#,
        ];
        let outputs = [
            r#"{1: "one", 2: "two"}"#,
            r#"{"one": (0 + 1), "two": (10 - 8), "three": (15 / 5)}"#,
        ];
        for (input, output) in inputs.iter().zip(&outputs) {
            let parsed = &parse_program(&input).unwrap()[0];
            assert_eq!(&format! {"{}", parsed}, output);
            let ev = evaluated(&input);

            match ev {
                Object::Hash(_) => assert!(true),
                _ => assert!(false),
            }
        }

        let inputs = [
            r#"let a = {1: "one", 2: "two"}; a[1]"#,
            r#"let bar = {"one": (0 + 1), "two": (10 - 8), "three": (15 / 5)}; bar["two"]"#,
        ];
        let outputs = [r#""one""#, "2"];
        for (input, output) in inputs.iter().zip(&outputs) {
            let ev = evaluated(&input);
            assert_eq!(&format! {"{}", ev}, output);
        }
    }

    #[test]
    fn test_method() {
        let inputs = [r#"a.foo(1 + 2 * 3)"#, r#"bar.foo_bar(2, 3)"#, "bar.foo"];
        let outputs = [r#"a.foo((1 + (2 * 3)))"#, "bar.foo_bar(2, 3)", "bar.foo"];

        for (input, output) in inputs.iter().zip(&outputs) {
            let parsed = &parse_program(&input).unwrap()[0];
            assert_eq!(&format! {"{}", parsed}, output);
        }
    }
}
