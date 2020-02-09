use crate::Object;
use std::collections::HashMap;

pub struct Environment {
    store: HashMap<&'static str, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        let mut store: HashMap<&'static str, Object> = HashMap::new();
        Environment { store }
    }
}

impl Environment {
    pub fn set(key: &'static str) {}
}
