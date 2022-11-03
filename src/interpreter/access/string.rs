use std::{cell::RefCell, rc::Rc};

use num::BigInt;

use crate::interpreter::state::{Value, ValueRef};

use super::access_error::access_error;

pub fn access_string(value: ValueRef, name: &str) -> ValueRef {
    Rc::new(RefCell::new(match name {
        "len" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match *args[0].borrow() {
                Value::String(s) => Rc::new(RefCell::new(Value::Integer(BigInt::from(s.len())))),
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => Rc::new(RefCell::new(Value::Error(
                "Invalid number of arguments".to_string(),
            ))),
        }),
        "to_int" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match *args[0].borrow() {
                Value::String(s) => match s.parse::<BigInt>() {
                    Ok(f) => Rc::new(RefCell::new(Value::Integer(f))),
                    Err(_) => Rc::new(RefCell::new(Value::Error(
                        "Failed to parse float".to_string(),
                    ))),
                },
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => Rc::new(RefCell::new(Value::Error(
                "Invalid number of arguments".to_string(),
            ))),
        }),
        "to_float" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match *args[0].borrow() {
                Value::String(s) => match s.parse::<f64>() {
                    Ok(f) => Rc::new(RefCell::new(Value::Float(f))),
                    Err(_) => Rc::new(RefCell::new(Value::Error(
                        "Failed to parse float".to_string(),
                    ))),
                },
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => Rc::new(RefCell::new(Value::Error(
                "Invalid number of arguments".to_string(),
            ))),
        }),
        "is_empty" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match *args[0].borrow() {
                Value::String(s) => Rc::new(RefCell::new(Value::Boolean(s.is_empty()))),
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => Rc::new(RefCell::new(Value::Error(
                "Invalid number of arguments".to_string(),
            ))),
        }),
        "to_string" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match *args[0].borrow() {
                Value::String(s) => Rc::new(RefCell::new(Value::String(s.clone()))),
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => Rc::new(RefCell::new(Value::Error(
                "Invalid number of arguments".to_string(),
            ))),
        }),
        _ => access_error("String", &name),
    }))
}
