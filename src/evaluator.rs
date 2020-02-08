use crate::ast::{Expression, Program, Statement};
use crate::object::Object;

pub fn eval(program_ast: &Program) -> Vec<Object> {
    program_ast.iter().map(eval_stmt).collect()
}

fn eval_stmt(stmt: &Statement) -> Object {
    match stmt {
        Statement::Expr(expr) => eval_expr(expr),
        _ => Object::Null,
    }
}

fn eval_expr(expr: &Expression) -> Object {
    match expr {
        Expression::IntegerLiteral(int) => Object::Int(*int),
        Expression::Bool(b) => Object::Bool(*b),
        Expression::Prefix { operator, expr } => {
            let right = eval_expr(expr);
            eval_prefix_expr(operator, &right)
        }
        _ => Object::Null,
    }
}

fn eval_prefix_expr(operator: &str, right: &Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expr(right),
        "-" => eval_minus_prefix_expr(right),
        _ => Object::Null,
    }
}

fn eval_bang_operator_expr(right: &Object) -> Object {
    match right {
        Object::Bool(b) => Object::Bool(!*b),
        _ => Object::Null,
    }
}

fn eval_minus_prefix_expr(right: &Object) -> Object {
    match right {
        Object::Int(int) => Object::Int(-*int),
        _ => Object::Null,
    }
}
