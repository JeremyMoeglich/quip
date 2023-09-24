use crate::state::{program_state::ProgramState, value_ref::ValueRef, value::value::Value};


pub fn interpret_variable(name: &str, state: &ProgramState) -> ValueRef {
    state
        .get_variable(name)
        .unwrap_or(ValueRef::new(Value::Error(format!(
            "Variable '{}' not foundF",
            name
        ))))
}
