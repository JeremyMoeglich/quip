use num::BigInt;

use crate::interpreter::state::Value;

use super::access_error::access_error;

pub fn access_string(value: String, name: &str) -> Value {
    match name {
        "len" => Value::NativeFunction(vec![Value::String(value)], |args| match args.len() {
            1 => match &args[0] {
                Value::String(ref s) => Value::Integer(BigInt::from(s.len())),
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => Value::Error("Invalid number of arguments".to_string()),
        }),
        "to_int" => Value::NativeFunction(vec![Value::String(value)], |args| match args.len() {
            1 => match &args[0] {
                Value::String(ref s) => match s.parse::<BigInt>() {
                    Ok(i) => Value::Integer(i),
                    Err(_) => Value::Error("Failed to parse integer".to_string()),
                },
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => Value::Error("Invalid number of arguments".to_string()),
        }),
        "to_float" => Value::NativeFunction(vec![Value::String(value)], |args| match args.len() {
            1 => match &args[0] {
                Value::String(ref s) => match s.parse::<f64>() {
                    Ok(i) => Value::Float(i),
                    Err(_) => Value::Error("Failed to parse float".to_string()),
                },
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => Value::Error("Invalid number of arguments".to_string()),
        }),
        "is_empty" => Value::NativeFunction(vec![Value::String(value)], |args| match args.len() {
            1 => match &args[0] {
                Value::String(ref s) => Value::Boolean(s.is_empty()),
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => Value::Error("Invalid number of arguments".to_string()),
        }),
        "to_string" => Value::NativeFunction(vec![Value::String(value)], |args| match args.len() {
            1 => match &args[0] {
                Value::String(ref s) => Value::String(s.to_string()),
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => Value::Error("Invalid number of arguments".to_string()),
        }),
        _ => access_error("String", &name),
    }
}
