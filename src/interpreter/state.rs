use std::{cell::RefCell, collections::HashMap, rc::Rc};

use num::BigInt;

use super::function::Function;

#[derive(Debug, Clone)]
pub struct ProgramScope {
    pub variables: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ProgramState {
    pub scopes: Vec<Rc<RefCell<ProgramScope>>>,
}

impl ProgramState {
    pub fn new() -> Self {
        Self { scopes: vec![] }
    }
    pub fn run_function(&self, name: &str, args: Vec<Value>) -> Value {
        let value = self.get_variable(name).expect("Function not found");
        match value {
            Value::Function(func) => func.call(args),
            _ => panic!("Only functions can be called"),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.borrow().variables.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn variable_exists(&self, name: &str) -> bool {
        self.get_variable(name).is_some()
    }

    pub fn new_scope(&self) -> Self {
        let mut new_state = self.clone();
        new_state.scopes.push(Rc::new(RefCell::new(ProgramScope {
            variables: HashMap::new(),
        })));
        new_state
    }

    pub fn set_variable(&mut self, name: &str, value: Value) {
        self.scopes
            .last_mut()
            .unwrap()
            .borrow_mut()
            .variables
            .insert(name.to_string(), value);
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(BigInt),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
    Function(Function),
}
