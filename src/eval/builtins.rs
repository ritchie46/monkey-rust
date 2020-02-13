use crate::Object;
use std::collections::HashMap;

lazy_static! {
    pub static ref BUILTINS: HashMap<String, BuiltinFn> = {
        let mut m = HashMap::new();
        m.insert("len".to_string(), len as BuiltinFn);
        m.insert("puts".to_string(), print as BuiltinFn);
        m
    };
}

pub type BuiltinFn = fn(Vec<Object>) -> Object;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Builtin {
    pub identifier: String,
    pub function: BuiltinFn,
}

pub fn len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::new_error(&format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }
    let arg = &args[0];
    match arg {
        Object::String(s) => Object::Int(s.len() as i64),
        Object::Array(v) => Object::Int(v.len() as i64),
        _ => Object::new_error("invalid argument type for builtin: len()"),
    }
}

pub fn print(args: Vec<Object>) -> Object {
    for (i, o) in args.iter().enumerate() {
        if i > 0 {
            print!(" ")
        }
        print!("{}", o);
    }
    Object::Ignore
}
