use crate::interpreter::state::value::value::Value;

pub fn access_error(value_type: &str, name: &str) -> Value {
    Value::Error(format!(
        "Invalid property access: {} does not have property {}",
        value_type, name
    ))
}
