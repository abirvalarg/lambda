use std::{collections::{HashMap, LinkedList}, cell::RefCell, rc::Rc};

use crate::value::Value;

#[derive(Debug)]
pub struct State {
    globals: HashMap<String, Rc<RefCell<Value>>>,
    scope: LinkedList<HashMap<String, Rc<RefCell<Value>>>>
}

impl State {
    pub fn new() -> Self {
        State {
            globals: HashMap::new(),
            scope: LinkedList::new()
        }
    }

    pub fn globals_mut(&mut self) -> &mut HashMap<String, Rc<RefCell<Value>>> {
        &mut self.globals
    }

    pub fn push_scope(&mut self, scope: HashMap<String, Rc<RefCell<Value>>>) {
        self.scope.push_front(scope);
    }

    pub fn pop_scope(&mut self) {
        self.scope.pop_front();
    }

    pub fn local(&self, name: &str) -> Option<Rc<RefCell<Value>>> {
        for scope in &self.scope {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        self.globals.get(name).map(Rc::clone)
    }
}
