use crate::state::{value::value::Value, value_ref::ValueRef};

use super::access_error::access_error;

pub fn access_list(value: ValueRef, name: &str) -> ValueRef {
    ValueRef::new(match name {
        "to_string" => Value::NativeFunction(vec![value], |args| match args.len() {
            1 => match &*args[0].get() {
                Value::List(s) => ValueRef::new(Value::String(format!(
                    "[{}]",
                    s.iter()
                        .map(|x| x.get_property("to_string").try_call(vec![]))
                        .map(|x| match &*x.get() {
                            Value::String(s) => s.clone(), // TODO: Handle errors
                            _ => panic!("Expected string"),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                ))),
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => ValueRef::new(Value::Error("Invalid number of arguments".to_string())),
        }),
        "map" => Value::NativeFunction(vec![value], |args| match args.len() {
            2 => match &*args[0].get() {
                Value::List(lst) => {
                    let func = &args[1];
                    ValueRef::new(Value::List(
                        lst.iter().map(|x| func.try_call(vec![x.clone()])).collect(),
                    ))
                }
                _ => unreachable!("Internal error, invalid argument type"),
            },
            _ => ValueRef::new(Value::Error("Invalid number of arguments".to_string())),
        }),
        _ => access_error("String", &name),
    })
}
