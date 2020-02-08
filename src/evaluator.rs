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
        _ => Object::Null,
    }
}
