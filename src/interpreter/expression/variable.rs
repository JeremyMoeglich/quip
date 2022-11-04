use crate::interpreter::state::{
    program_state::ProgramState, value::value::Value, value_ref::ValueRef,
};

pub fn interpret_variable(name: &str, state: &ProgramState) -> ValueRef {
    state
        .get_variable(name)
        .unwrap_or(ValueRef::new(Value::Error(format!(
            "Variable '{}' not found",
            name
        ))))
}
