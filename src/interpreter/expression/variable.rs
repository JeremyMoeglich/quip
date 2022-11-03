use crate::interpreter::state::{new_value_ref, ProgramState, Value, ValueRef};

pub fn interpret_variable(name: &str, state: &ProgramState) -> ValueRef {
    state
        .get_variable(name)
        .unwrap_or(new_value_ref(Value::Error(format!(
            "Variable '{}' not found",
            name
        ))))
}
