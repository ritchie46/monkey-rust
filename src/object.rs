use std::fmt;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Null,
}

impl Object {
    pub fn get_type(&self) -> &'static str {
        match self {
            Object::Int(_) => "int",
            Object::Bool(_) => "bool",
            Object::Null => "null",
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(int) => write!(f, "{}", int),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Null => f.write_str("null"),
            _ => f.write_str("not impl."),
        }
    }
}
