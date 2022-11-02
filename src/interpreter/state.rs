use std::collections::HashMap;

use num::BigInt;

use super::function::Function;

#[derive(Debug, Clone)]
pub struct ProgramState {
    pub variables: HashMap<String, Value>,
}

impl ProgramState {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
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

