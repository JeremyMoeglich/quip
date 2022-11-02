use crate::interpreter::state::{ProgramState, Value};

pub fn interpret_variable(name: String, state: &ProgramState) -> Value {
    state.variables.get(&name).unwrap().clone()
}