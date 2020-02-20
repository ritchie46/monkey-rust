use fnv::FnvHashMap as HashMap;
use std::borrow::Borrow;
use std::collections::hash_map::Entry;

const GLOBAL_SCOPE: &'static str = "global";

pub struct Symbol<'cmpl> {
    scope: &'cmpl str,
    pub index: usize,
}

pub struct SymbolTable<'cmpl> {
    store: HashMap<String, Symbol<'cmpl>>,
    num_definitions: usize,
}

impl<'cmpl> SymbolTable<'cmpl> {
    pub fn new() -> SymbolTable<'cmpl> {
        let store = HashMap::default();
        SymbolTable {
            store,
            num_definitions: 0,
        }
    }

    pub fn define(&mut self, name: String) -> &Symbol<'cmpl> {
        let smbl = Symbol {
            scope: GLOBAL_SCOPE,
            index: self.num_definitions,
        };
        // Insert whatsover
        let smbl = match self.store.entry(name) {
            Entry::Vacant(entry) => entry.insert(smbl),
            Entry::Occupied(mut entry) => {
                entry.insert(smbl);
                entry.into_mut()
            }
        };

        self.num_definitions += 1;
        smbl
    }

    pub fn resolve(&self, name: &'cmpl str) -> Option<&Symbol<'cmpl>> {
        self.store.get(name)
    }
}
