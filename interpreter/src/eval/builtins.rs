use crate::eval::environment::Env;
use crate::eval::object::Object;
use std::collections::HashMap;
use std::io::Write;

lazy_static! {
    pub static ref BUILTINS: HashMap<String, BuiltinFn> = {
        let mut m = HashMap::new();
        m.insert("len".to_string(), len as BuiltinFn);
        m.insert("print".to_string(), print as BuiltinFn);
        m.insert("insert".to_string(), insert as BuiltinFn);
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
        if o.get_type() == "str" {
            let s = format!("{}", o);

            print!("{}", (s[1..s.len() - 1]).to_string())
        } else {
            print!("{}", o);
        }
    }
    print!("\n");
    std::io::stdout().flush();
    Object::Ignore
}

pub fn insert(args: Vec<Object>) -> Object {
    if args.len() != 3 {
        return Object::new_error("expected a container, and index/key and a value");
    }
    let mut args = args.into_iter();
    let mut container = args.next().unwrap();
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    match container {
        Object::Hash(_) => container.insert_hash_value(key, value),
        _ => Object::new_error("not supported"),
    }
}
