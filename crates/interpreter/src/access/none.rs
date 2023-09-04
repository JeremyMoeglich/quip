use crate::interpreter::state::{value::value::Value, value_ref::ValueRef};

use super::access_error::access_error;

pub fn access_none(value: ValueRef, name: &str) -> ValueRef {
    ValueRef::new(match name {
        "to_string" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match &*args[0].get() {
                Value::None => ValueRef::new(Value::String("None".to_string())),
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => ValueRef::new(Value::Error("Invalid number of arguments".to_string())),
        }),
        _ => access_error("String", &name),
    })
}
