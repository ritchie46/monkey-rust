use std::fmt;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Null,
    ReturnValue(Box<Object>),
    Error(String),
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
            _ => "null",
        }
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
            _ => f.write_str("not impl."),
        }
    }
}
