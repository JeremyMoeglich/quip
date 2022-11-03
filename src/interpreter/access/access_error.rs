use crate::interpreter::state::Value;

pub fn access_error(value_type: &str, name: &str) -> Value {
    Value::Error(format!(
        "Invalid property access: {} does not have property {}",
        value_type, name
    ))
}
