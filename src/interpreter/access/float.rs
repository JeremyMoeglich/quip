use std::{cell::RefCell, rc::Rc};

use crate::interpreter::state::{Value, ValueRef};

use super::access_error::access_error;

pub fn access_float(value: ValueRef, name: &str) -> ValueRef {
    Rc::new(RefCell::new(match name {
        "to_string" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match *args[0].borrow() {
                Value::Float(s) => Rc::new(RefCell::new(Value::String(s.to_string()))),
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => Rc::new(RefCell::new(Value::Error(
                "Invalid number of arguments".to_string(),
            ))),
        }),
        _ => access_error("String", &name),
    }))
}
