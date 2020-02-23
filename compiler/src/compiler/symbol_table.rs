use crate::code::OpCode;
use fnv::FnvHashMap as HashMap;
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::collections::hash_map::Entry;
use std::rc::Rc;

#[derive(Clone)]
pub enum Scope {
    Global,
    Local,
}

#[derive(Clone)]
pub struct Symbol {
    pub scope: Scope,
    pub index: usize,
}

pub struct SymbolTable {
    pub outer: Option<Rc<RefCell<SymbolTable>>>,
    store: HashMap<String, Symbol>,
    num_definitions: usize,
}

impl SymbolTable {
    pub fn new() -> Rc<RefCell<SymbolTable>> {
        let store = HashMap::default();
        Rc::new(RefCell::new(SymbolTable {
            outer: None,
            store,
            num_definitions: 0,
        }))
    }

    pub fn new_enclosed(outer: Rc<RefCell<SymbolTable>>) -> Rc<RefCell<SymbolTable>> {
        let s = SymbolTable::new();
        {
            let mut tbl = s.borrow_mut();
            tbl.outer = Some(outer);
        }
        s
    }

    pub fn define(&mut self, name: String) -> Symbol {
        let scope;
        if self.outer.is_none() {
            scope = Scope::Global;
        } else {
            scope = Scope::Local;
        }
        let smbl = Symbol {
            scope,
            index: self.num_definitions,
        };
        // symbol clones are cheap
        self.store.insert(name, smbl.clone());

        self.num_definitions += 1;
        smbl
    }

    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        if let Some(smbl) = self.store.get(name) {
            return Some(smbl.clone());
        }

        match &self.outer {
            Some(smbl_table) => {
                let tbl = RefCell::borrow(smbl_table);
                tbl.resolve(name)
            }
            None => None,
        }
    }
}
