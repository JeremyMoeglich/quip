use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::value_ref::ValueRef;

#[derive(Debug, Clone)]
pub struct ProgramScope {
    pub variables: HashMap<String, ValueRef>,
}

#[derive(Debug, Clone)]
pub struct ProgramState {
    pub scopes: Vec<Rc<RefCell<ProgramScope>>>,
}

impl ProgramState {
    pub fn new() -> Self {
        Self { scopes: vec![] }
    }

    pub fn get_variable(&self, name: &str) -> Option<ValueRef> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.borrow().variables.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn new_scope(&self) -> Self {
        let mut new_state = self.clone();
        new_state.scopes.push(Rc::new(RefCell::new(ProgramScope {
            variables: HashMap::new(),
        })));
        new_state
    }

    pub fn set_new_variable(&self, name: &str, value: ValueRef) {
        self.scopes
            .last()
            .unwrap()
            .borrow_mut()
            .variables
            .insert(name.to_string(), value);
    }

    pub fn replace_variable(&self, name: &str, value: ValueRef) {
        for scope in self.scopes.iter().rev() {
            if scope.borrow().variables.contains_key(name) {
                scope.borrow_mut().variables.insert(name.to_string(), value);
                return;
            }
        }
        panic!("Variable {} not found", name); // TODO: Handle Error
    }
}
