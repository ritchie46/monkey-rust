use crate::eval::builtins::{Builtin, BuiltinFn};
use crate::format;
use crate::parser::ast::{Expression, Statement};
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
    String(String),
    Builtin(Builtin),
    Array(Box<Vec<Object>>),
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
            (Object::String(a), Object::String(b)) => a == b,
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
            Object::String(_) => "str",
            Object::Builtin(_) => "builtin",
            Object::Array(_) => "array",
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

    pub fn new_builtin(identifier: &str, function: BuiltinFn) -> Object {
        let builtin = Builtin {
            identifier: identifier.to_string(),
            function,
        };
        Object::Builtin(builtin)
    }

    pub fn new_array(values: Vec<Object>) -> Object {
        Object::Array(Box::new(values))
    }

    pub fn index_array(&self, index: i64) -> Object {
        let a = match self {
            Object::Array(a) => a,
            o => {
                return Object::new_error(&format!(
                    "index operator not supported on: {}",
                    o
                ))
            }
        };

        let len = a.len() as i64;

        let i = if index >= 0 && index < len {
            index as usize
        }
        // negative indexing
        else if index < 0 && index.abs() <= len {
            (len + index) as usize
        } else {
            return Object::new_error(&format!(
                "index value outside the array's range: {}, index: {}",
                len, index
            ));
        };
        a[i].clone()
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
                f.write_str(&format::fmt_function_literal(&func.parameters, &func.body))
            }
            Object::String(s) => write!(f, r#""{}""#, s),
            Object::Builtin(b) => write!(f, "builtin: {}", b.identifier),
            Object::Array(values) => f.write_str(&format::fmt_array_literal(values)),
            _ => f.write_str("not impl."),
        }
    }
}