use crate::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Environment {
    // Cannot borrow identifier from the ast.
    // In the repl the ast does not live long enough.
    // Goes out of scope every evaluation.
    store: HashMap<String, Object>,
    // Box not needed as Rc<T> already allocates on the heap.
    outer: Option<Env>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Environment>> {
        let mut store: HashMap<String, Object> = HashMap::new();
        let env = Environment { store, outer: None };
        Rc::new(RefCell::new(env))
    }
    pub fn set(&mut self, identifier: &str, value: &Object) {
        self.store.insert(identifier.to_string(), value.clone());
    }

    pub fn get(&self, identifier: &str) -> Option<Object> {
        let q = self.store.get(identifier);

        match q {
            Some(obj) => Some(obj.clone()),
            None => self.get_from_outer(identifier),
        }
    }

    fn get_from_outer(&self, identifier: &str) -> Option<Object> {
        let outer_env = match &self.outer {
            None => return None,
            Some(env) => env.borrow(),
        };
        outer_env.get(identifier)
    }
}

pub fn new_enclosed_environment(outer: &Env) -> Env {
    let mut store: HashMap<String, Object> = HashMap::new();
    let env = Environment {
        store,
        outer: Some(Rc::clone(outer)),
    };
    Rc::new(RefCell::new(env))
}
