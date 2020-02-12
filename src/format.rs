use crate::parser::ast::{Expression, Statement};
use std::fmt;

/// Helper functions for Expression, Statement and Object formatting

pub fn fmt_block(stmts: &Vec<Statement>) -> String {
    let mut s = String::new();
    for b in stmts {
        s.push_str(&format!("{}", b))
    }
    s
}

pub fn fmt_alternative_block(alt: &Option<Box<Statement>>) -> String {
    match alt {
        Some(s) => format!("{}", s),
        None => "pass".to_string(),
    }
}

fn fmt_comma_separated_expr<T: fmt::Display>(s: &mut String, args: &[T]) {
    for (i, p) in args.iter().enumerate() {
        if i == 0 {
            s.push_str(&format!("{}", p))
        } else {
            s.push_str(&format!(", {}", p))
        }
    }
}

pub fn fmt_function_literal<T: fmt::Display>(args: &[T], body: &Statement) -> String {
    let mut s = "fn(".to_string();
    fmt_comma_separated_expr(&mut s, args);
    s.push_str(&format!(") {{ {} }}", body));
    s
}

pub fn fmt_call_expr(function: &Expression, args: &Vec<Expression>) -> String {
    let mut s = format!("{}(", function);
    fmt_comma_separated_expr(&mut s, args);
    s.push(')');
    s
}

pub fn fmt_array_literal<T: fmt::Display>(val: &[T]) -> String {
    let mut s = "[".to_string();
    fmt_comma_separated_expr(&mut s, val);
    s.push(']');
    s
}
