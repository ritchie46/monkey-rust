use std::fmt;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Null,
    ReturnValue(Box<Object>),
}

impl Object {
    pub fn new_return_val(obj: Object) -> Object {
        Object::ReturnValue(Box::new(obj))
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(int) => write!(f, "{}", int),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Null => f.write_str("null"),
            Object::ReturnValue(obj) => write!(f, "{}", obj),
            _ => f.write_str("not impl."),
        }
    }
}
