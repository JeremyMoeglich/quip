use crate::interpreter::state::Value;

pub fn access_error(value_type: &str, name: &str) -> Value {
    Value::Error(format!("Invalid access: {} does not have a field named {}", value_type, name))
}