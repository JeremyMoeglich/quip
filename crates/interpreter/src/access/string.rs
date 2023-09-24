use num::BigInt;

use crate::state::{value::value::Value, value_ref::ValueRef};

use super::access_error::access_error;

pub fn access_string(value: ValueRef, name: &str) -> ValueRef {
    ValueRef::new(match name {
        "len" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match &*args[0].get() {
                Value::String(s) => ValueRef::new(Value::Integer(BigInt::from(s.len()))),
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => ValueRef::new(Value::Error("Invalid number of arguments".to_string())),
        }),
        "to_int" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match &*args[0].get() {
                Value::String(s) => match s.parse::<BigInt>() {
                    Ok(f) => ValueRef::new(Value::Integer(f)),
                    Err(_) => ValueRef::new(Value::Error("Failed to parse float".to_string())),
                },
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => ValueRef::new(Value::Error("Invalid number of arguments".to_string())),
        }),
        "to_float" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match &*args[0].get() {
                Value::String(s) => match s.parse::<f64>() {
                    Ok(f) => ValueRef::new(Value::Float(f)),
                    Err(_) => ValueRef::new(Value::Error("Failed to parse float".to_string())),
                },
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => ValueRef::new(Value::Error("Invalid number of arguments".to_string())),
        }),
        "is_empty" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match &*args[0].get() {
                Value::String(s) => ValueRef::new(Value::Boolean(s.is_empty())),
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => ValueRef::new(Value::Error("Invalid number of arguments".to_string())),
        }),
        "to_string" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match &*args[0].get() {
                Value::String(s) => ValueRef::new(Value::String(s.clone())),
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => ValueRef::new(Value::Error("Invalid number of arguments".to_string())),
        }),
        _ => access_error("String", &name),
    })
}
