use crate::ast::{fmt_function_literal, Expression, Statement};
use crate::Env;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Null,
    ReturnValue(Box<Object>),
    Error(String),
    Function(Function),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Expression>, // Identifier
    pub body: Statement,             // Blockstmt
    env: Env,
}

impl PartialEq<Object> for Object {
    fn eq(&self, other: &Object) -> bool {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => a == b,
            (Object::Bool(a), Object::Bool(b)) => a == b,
            (Object::Null, Object::Null) => true,
            (Object::Error(a), Object::Error(b)) => a == b,
            _ => false,
        }
    }
}

impl Object {
    pub fn new_return_val(obj: Object) -> Object {
        Object::ReturnValue(Box::new(obj))
    }

    pub fn new_error(s: &str) -> Object {
        Object::Error(format!("Error: {}", s))
    }

    pub fn get_type(&self) -> &'static str {
        match self {
            Object::Int(_) => "int",
            Object::Bool(_) => "bool",
            Object::Error(_) => "err",
            _ => "null",
        }
    }

    pub fn new_function(
        parameters: &Vec<Expression>,
        body: &Statement,
        env: &Env,
    ) -> Object {
        Object::Function(Function {
            parameters: parameters.clone(),
            body: body.clone(),
            env: Rc::clone(env),
        })
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(int) => write!(f, "{}", int),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Null => f.write_str("null"),
            Object::ReturnValue(obj) => write!(f, "{}", obj),
            Object::Error(s) => f.write_str(s),
            Object::Function(func) => {
                f.write_str(&fmt_function_literal(&func.parameters, &func.body))
            }
            _ => f.write_str("not impl."),
        }
    }
}
