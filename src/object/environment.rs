use crate::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone)]
pub struct Environment {
    // Cannot borrow identifier from the ast.
    // In the repl the ast does not live long enough.
    // Goes out of scope every evaluation.
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Environment>> {
        let mut store: HashMap<String, Object> = HashMap::new();
        let env = Environment { store };
        Rc::new(RefCell::new(env))
    }
    pub fn set(&mut self, identifier: &str, value: Object) {
        self.store.insert(identifier.to_string(), value);
    }

    pub fn get(&self, identifier: &str) -> Option<&Object> {
        self.store.get(identifier)
    }
}
