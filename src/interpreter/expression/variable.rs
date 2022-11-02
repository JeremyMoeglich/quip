use std::{cell::RefCell, rc::Rc};

use crate::interpreter::state::{ProgramState, Value};

pub fn interpret_variable(name: &str, state: &ProgramState) -> Value {
    state
        .borrow()
        .get_variable(name)
        .expect("Variable does not exist")
}
