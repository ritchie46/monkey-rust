use crate::eval::builtins::{Builtin, BuiltinFn};
use crate::format;
use crate::parser::ast::{Expression, Statement};
use crate::Env;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Function {
    pub parameters: Vec<Expression>, // Identifier
    pub body: Statement,             // Blockstmt
    env: Env,
}

#[derive(Debug, Clone, Eq)]
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
    Hash(Rc<RefCell<HashMap<Object, Object>>>),
    Ignore,
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

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::Int(v) => v.hash(state),
            Object::Bool(v) => v.hash(state),
            Object::String(v) => v.hash(state),
            o => panic!(format!("cannot hash {}", o)),
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
            Object::Hash(_) => "hash",
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
    pub fn new_hash(keys: Vec<Object>, values: Vec<Object>) -> Object {
        let mut map = HashMap::new();
        for (k, v) in keys.into_iter().zip(values) {
            map.insert(k, v);
        }
        Object::Hash(Rc::new(RefCell::new(map)))
    }

    pub fn get_hash_value(&self, key: Object) -> Object {
        let mut map = match self {
            Object::Hash(m) => m.borrow(),
            _ => {
                return Object::new_error(&format!(
                    "index operator `{}` not supported on: {}",
                    key.get_type(),
                    self.get_type()
                ))
            }
        };
        let value = map.get(&key);
        match value {
            Some(v) => v.clone(),
            None => Object::new_error(&format!("key: {} not found", key)),
        }
    }

    pub fn insert_hash_value(&mut self, key: Object, value: Object) -> Object {
        let mut map = match self {
            Object::Hash(m) => m.borrow_mut(),
            _ => {
                return Object::new_error(&format!(
                    "index operator `{}` not supported on: {}",
                    key.get_type(),
                    self.get_type()
                ))
            }
        };
        map.insert(key, value);
        Object::Ignore
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
            Object::Hash(map) => f.write_str(&format::fmt_hash_literal(
                &map.borrow()
                    .keys()
                    .into_iter()
                    .map(|a| a.clone())
                    .collect::<Vec<Object>>(),
                &map.borrow()
                    .values()
                    .into_iter()
                    .map(|a| a.clone())
                    .collect::<Vec<Object>>(),
            )),
            Object::Ignore => f.write_str(""),
            _ => f.write_str("not impl."),
        }
    }
}
