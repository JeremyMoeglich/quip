mod access_error;
mod boolean;
mod float;
mod integer;
mod list;
mod none;
mod string;

use super::state::{value::value::Value, value_ref::ValueRef};

pub fn interpret_access(left: ValueRef, right: &str) -> ValueRef {
    match &*left.resolve().get() {
        Value::String(_) => string::access_string(left.clone(), right),
        Value::Integer(_) => integer::access_integer(left.clone(), right),
        Value::Float(_) => float::access_float(left.clone(), right),
        Value::Boolean(_) => boolean::access_boolean(left.clone(), right),
        Value::List(_) => list::access_list(left.clone(), right),
        Value::None => none::access_none(left.clone(), right),
        Value::Error(_) => left.clone(),
        Value::NativeFunction(_, _) => {
            ValueRef::new(Value::Error("Functions do not have properties".to_string()))
        }
        Value::Function(_) => {
            ValueRef::new(Value::Error("Functions do not have properties".to_string()))
        }
        Value::Reference(_) => unreachable!("Reference should have been resolved"),
    }
}
