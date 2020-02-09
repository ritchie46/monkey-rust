use crate::ast::{Expression, Program, Statement};
use crate::{Environment, Object};
use std::fs::read_to_string;
use std::process::id;

/// Run all statements and return last
pub fn eval_program(program_ast: &Program, env: &mut Environment) -> Object {
    let mut stmts_executed = vec![];

    for stmt in program_ast {
        let result = eval_stmt(stmt, env);

        // A return or an Error should stop further evaluation
        // The return is unpacked
        match result {
            Object::ReturnValue(obj) => return *obj,
            Object::Error(_) => return result,
            _ => stmts_executed.push(result),
        }
    }
    stmts_executed.pop().unwrap()
}

fn eval_block_stmt(block: &Vec<Statement>, env: &mut Environment) -> Object {
    let mut result: Object = Object::Null;
    for stmt in block {
        result = eval_stmt(stmt, env);

        match result {
            Object::Error(_) => return result,
            // Don't unpack. but return the Return Wrapper.
            // Unpacking is done in eval_program
            Object::ReturnValue(_) => return result,
            _ => {}
        }
    }
    result
}

fn eval_stmt(stmt: &Statement, env: &mut Environment) -> Object {
    match stmt {
        Statement::Expr(expr) => eval_expr(expr, env),
        Statement::Block(stmts) => eval_block_stmt(stmts, env),
        Statement::Return(expr) => Object::new_return_val(eval_expr(expr, env)),
        Statement::Let(ident, expr) => eval_let_stmt(ident, expr, env),
        _ => Object::Null,
    }
}

fn eval_expr(expr: &Expression, env: &mut Environment) -> Object {
    match expr {
        Expression::IntegerLiteral(int) => Object::Int(*int),
        Expression::Bool(b) => Object::Bool(*b),
        Expression::Prefix { operator, expr } => {
            let right = eval_expr(expr, env);
            eval_prefix_expr(operator, &right)
        }
        Expression::Infix {
            left,
            operator,
            right,
        } => {
            let left = eval_expr(left, env);
            let right = eval_expr(right, env);
            eval_infix_expr(operator, &left, &right)
        }
        Expression::IfExpression {
            condition,
            consequence,
            alternative,
        } => eval_if_expr(condition, consequence, alternative, env),
        Expression::Identifier(name) => eval_identifier(name, env),
        _ => Object::Null,
    }
}

fn eval_prefix_expr(operator: &str, right: &Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expr(right),
        "-" => eval_minus_prefix_expr(right),
        _ => Object::new_error(&format!("unknown operator: {} {}", operator, right)),
    }
}

fn eval_bang_operator_expr(right: &Object) -> Object {
    match right {
        Object::Bool(b) => Object::Bool(!*b),
        _ => Object::new_error(&format!("unknown operator: !{}", right.get_type())),
    }
}

fn eval_minus_prefix_expr(right: &Object) -> Object {
    match right {
        Object::Int(int) => Object::Int(-*int),
        _ => Object::new_error(&format!("unknown operator: -{}", right.get_type())),
    }
}

fn eval_infix_expr(operator: &str, left: &Object, right: &Object) -> Object {
    match (left, right) {
        (Object::Int(l), Object::Int(r)) => eval_int_infix_expr(operator, *l, *r),
        (Object::Bool(l), Object::Bool(r)) => eval_bool_infix_expr(operator, *l, *r),
        _ => Object::new_error(&format!(
            "type mismatch: {} {} {}",
            left.get_type(),
            operator,
            right.get_type()
        )),
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
        op => Object::new_error(&format!("unkown operator: int {} int", op)),
    }
}

fn eval_bool_infix_expr(operator: &str, left: bool, right: bool) -> Object {
    match operator {
        "==" => Object::Bool(left == right),
        "!=" => Object::Bool(left != right),
        op => Object::new_error(&format!("unknown operator: bool {} bool", op)),
    }
}

fn eval_if_expr(
    condition: &Expression,
    consequence: &Statement,
    alternative: &Option<Box<Statement>>,
    env: &mut Environment,
) -> Object {
    let condition = eval_expr(condition, env);
    if is_truthy(&condition) {
        eval_stmt(consequence, env)
    } else {
        match alternative {
            Some(stmt) => eval_stmt(stmt, env),
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

fn eval_let_stmt(identifier: &str, expr: &Expression, env: &mut Environment) -> Object {
    let evaluated = eval_expr(expr, env);
    if let Object::Error(_) = evaluated {
        return evaluated;
    }
    env.set(identifier, evaluated);
    Object::Null
}

fn eval_identifier(identifier: &str, env: &Environment) -> Object {
    let val = env.get(identifier);
    val.unwrap_or(&Object::new_error(&format!(
        "identifier not found: {}",
        identifier
    )))
    .clone()
}
