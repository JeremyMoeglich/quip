use crate::interpreter::state::Value;

use super::access_error::access_error;

pub fn access_boolean(value: bool, name: &str) -> Value {
    match name {
        "to_string" => {
            Value::NativeFunction(vec![Value::Boolean(value)], |args| match args.len() {
                1 => match &args[0] {
                    Value::Boolean(ref s) => Value::String(s.to_string()),
                    _ => unreachable!("Internal error, invalid argument type"),
                },
                _ => Value::Error("Invalid number of arguments".to_string()),
            })
        }
        _ => access_error("String", &name),
    }
}
