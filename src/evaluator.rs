use crate::ast::{Expression, Program, Statement};
use crate::object::Object;

/// Run all statements and return last
pub fn eval(program_ast: &Program) -> Object {
    let mut ran_program: Vec<Object> = program_ast.iter().map(eval_stmt).collect();
    ran_program.pop().unwrap()
}

fn eval_stmt(stmt: &Statement) -> Object {
    match stmt {
        Statement::Expr(expr) => eval_expr(expr),
        Statement::Block(stmts) => eval(stmts),
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
        Expression::Infix {
            left,
            operator,
            right,
        } => {
            let left = eval_expr(left);
            let right = eval_expr(right);
            eval_infix_expr(operator, &left, &right)
        }
        Expression::IfExpression {
            condition,
            consequence,
            alternative,
        } => eval_if_expr(condition, consequence, alternative),
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

fn eval_infix_expr(operator: &str, left: &Object, right: &Object) -> Object {
    match (left, right) {
        (Object::Int(l), Object::Int(r)) => eval_int_infix_expr(operator, *l, *r),
        (Object::Bool(l), Object::Bool(r)) => eval_bool_infix_expr(operator, *l, *r),
        _ => Object::Null,
    }
}

fn eval_int_infix_expr(operator: &str, left: i64, right: i64) -> Object {
    match operator {
        "+" => Object::Int(left + right),
        "-" => Object::Int(left - right),
        "*" => Object::Int(left * right),
        "/" => Object::Int(left / right),
        "<" => Object::Bool(left < right),
        ">" => Object::Bool(left > right),
        "==" => Object::Bool(left == right),
        "!=" => Object::Bool(left != right),
        _ => Object::Null,
    }
}

fn eval_bool_infix_expr(operator: &str, left: bool, right: bool) -> Object {
    match operator {
        "==" => Object::Bool(left == right),
        "!=" => Object::Bool(left != right),
        _ => Object::Null,
    }
}

fn eval_if_expr(
    condition: &Expression,
    consequence: &Statement,
    alternative: &Option<Box<Statement>>,
) -> Object {
    let condition = eval_expr(condition);
    if is_truthy(&condition) {
        eval_stmt(consequence)
    } else {
        match alternative {
            Some(stmt) => eval_stmt(stmt),
            _ => Object::Null,
        }
    }
}

fn is_truthy(condition: &Object) -> bool {
    match condition {
        Object::Null => false,
        Object::Bool(false) => false,
        _ => true,
    }
}
