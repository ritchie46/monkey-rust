use crate::eval::{
    builtins,
    builtins::{len, Builtin, BuiltinFn, BUILTINS},
    environment::{new_enclosed_environment, Env},
    object::{Function, Object},
};
use crate::parser::ast::{Expression, Program, Statement};

/// Run all statements and return last
pub fn eval_program(program_ast: &Program, env: &Env) -> Object {
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

fn eval_block_stmt(block: &[Statement], env: &Env) -> Object {
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

fn eval_stmt(stmt: &Statement, env: &Env) -> Object {
    match stmt {
        Statement::Expr(expr) => eval_expr(expr, env),
        Statement::Block(stmts) => eval_block_stmt(stmts, env),
        Statement::Return(expr) => Object::new_return_val(eval_expr(expr, env)),
        Statement::Let(ident, expr) => eval_let_stmt(ident, expr, env),
        _ => Object::Null,
    }
}

fn eval_expr(expr: &Expression, env: &Env) -> Object {
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
        Expression::FunctionLiteral { parameters, body } => {
            Object::new_function(parameters, body, env)
        }
        Expression::CallExpr {
            function: fn_literal,
            args,
        } => eval_call_expr(fn_literal, args, env),
        Expression::StringLiteral(s) => Object::String(s.clone()),
        Expression::ArrayLiteral(expressions) => eval_array_literal(expressions, env),
        Expression::IndexExpr { left, index } => eval_index_expr(left, index, env),
        Expression::HashLiteral { keys, values } => eval_hash_literal(keys, values, env),
        Expression::Method {
            left,
            identifier,
            args,
        } => eval_method_expr(left, identifier, args, env),
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
        (Object::Int(l), Object::Int(r)) => eval_int_infix_expr(operator, l, r),
        (Object::Bool(l), Object::Bool(r)) => eval_bool_infix_expr(operator, l, r),
        (Object::String(l), Object::String(r)) => eval_str_infix_expr(operator, l, r),
        _ => Object::new_error(&format!(
            "type mismatch: {} {} {}",
            left.get_type(),
            operator,
            right.get_type()
        )),
    }
}

fn eval_int_infix_expr(operator: &str, left: &i64, right: &i64) -> Object {
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

fn eval_bool_infix_expr(operator: &str, left: &bool, right: &bool) -> Object {
    match operator {
        "==" => Object::Bool(left == right),
        "!=" => Object::Bool(left != right),
        op => Object::new_error(&format!("unknown operator: bool {} bool", op)),
    }
}

fn eval_str_infix_expr(operator: &str, left: &str, right: &str) -> Object {
    match operator {
        "+" => Object::String(format!("{}{}", left, right)),
        op => Object::new_error(&format!("unknown operator: str {} str", op)),
    }
}

fn eval_if_expr(
    condition: &Expression,
    consequence: &Statement,
    alternative: &Option<Box<Statement>>,
    env: &Env,
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

fn eval_let_stmt(identifier: &str, expr: &Expression, env: &Env) -> Object {
    let evaluated = eval_expr(expr, env);
    if let Object::Error(_) = evaluated {
        return evaluated;
    }

    let mut env = env.borrow_mut();
    env.set(identifier, evaluated);
    Object::Ignore
}

fn eval_identifier(identifier: &str, env: &Env) -> Object {
    let env = env.borrow();

    let val = env.get(identifier);

    if val.is_some() {
        return val.unwrap();
    }
    let builtin = BUILTINS.get(identifier);

    if builtin.is_none() {
        return Object::new_error(&format!("identifier not found a: {}", identifier));
    }
    Object::new_builtin(identifier, *builtin.unwrap())
}

fn eval_call_expr(function: &Expression, args: &[Expression], env: &Env) -> Object {
    let function_ident = eval_expr(function, env);

    if function_ident.get_type() == "err" {
        return function_ident;
    }

    let arg_objs = eval_expressions(args, env);
    if arg_objs.len() == 1 {
        if let Object::Error(_) = arg_objs[0] {
            return arg_objs[0].clone();
        }
    }
    match function_ident {
        Object::Function(f) => apply_function(&f, arg_objs, env),
        Object::Builtin(b) => {
            let f = b.function;
            f(arg_objs)
        }
        _ => Object::new_error("function not defined"),
    }
}

fn eval_expressions(exprs: &[Expression], env: &Env) -> Vec<Object> {
    let mut iter = exprs.iter().map(|expr| eval_expr(expr, env));

    let mut objects: Vec<Object> = vec![];

    while let Some(o) = iter.next() {
        if let Object::Error(_) = o {
            return vec![o];
        }
        objects.push(o);
    }
    objects
}

fn apply_function(f: &Function, args: Vec<Object>, env: &Env) -> Object {
    //    if let Object::Function(f) = &func {
    let env = create_function_env(f, args, env);
    let evaluated = eval_stmt(&f.body, &env);

    if let Object::ReturnValue(return_val) = evaluated {
        return *return_val;
    }
    evaluated
}

fn create_function_env(func: &Function, args: Vec<Object>, env: &Env) -> Env {
    let env = new_enclosed_environment(env);

    {
        let mut envcell = env.borrow_mut();

        for (param, value) in func.parameters.iter().zip(args) {
            if let Expression::Identifier(ident) = param {
                envcell.set(ident, value)
            }
        }
    }
    env
}

fn eval_array_literal(exprs: &[Expression], env: &Env) -> Object {
    let vals = eval_expressions(exprs, env);
    Object::new_array(vals)
}

fn eval_index_expr(left: &Expression, index: &Expression, env: &Env) -> Object {
    let index = eval_expr(index, env);

    let obj = eval_expr(left, env);
    match &obj {
        Object::Hash(map) => obj.get_hash_value(index),
        Object::Array(arr) => index_array(obj, index),
        _ => {
            return Object::new_error(&format!(
                "index operator `{}` not supported on: {}",
                index.get_type(),
                obj.get_type()
            ))
        }
    }
}

fn index_array(array: Object, index: Object) -> Object {
    match index {
        // Already know that object is an array.
        Object::Int(i) => array.index_array(i),
        _ => Object::new_error(&format!(
            "index operator `{}` not supported on: {}",
            index.get_type(),
            array.get_type()
        )),
    }
}

fn eval_hash_literal(keys: &[Expression], values: &[Expression], env: &Env) -> Object {
    let keys = eval_expressions(keys, env);

    if keys.len() == 1 {
        if let Object::Error(_) = keys[0] {
            return keys[0].clone();
        }
    }
    let values = eval_expressions(values, env);
    if values.len() == 1 {
        if let Object::Error(_) = values[0] {
            return values[0].clone();
        }
    }
    Object::new_hash(keys, values)
}

fn eval_method_expr(
    left: &Expression,
    identifier: &Expression,
    args: &[Expression],
    env: &Env,
) -> Object {
    let left = eval_expr(left, env);

    match left {
        Object::Error(_) => return left,
        Object::Hash(_) => return call_hash_methods(left, identifier, args, env),
        _ => Object::Error(format!("method not found on {}", left.get_type())),
    }
}

fn call_hash_methods(
    left: Object,
    identifier: &Expression,
    args: &[Expression],
    env: &Env,
) -> Object {
    let method_name = match identifier {
        Expression::Identifier(s) => &s[..],
        _ => return Object::Error("not a valid method name".to_string()),
    };

    let mut args = eval_expressions(args, env);
    if args.len() == 1 {
        if let Object::Error(_) = args[0] {
            return args[0].clone();
        }
    }
    args.insert(0, left);

    match method_name {
        "insert" => builtins::insert(args),
        _ => Object::Error("method not found".to_string()),
    }
}
