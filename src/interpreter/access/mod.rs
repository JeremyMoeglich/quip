mod access_error;
mod boolean;
mod float;
mod integer;
mod list;
mod string;

use std::{cell::RefCell, rc::Rc};

use access_error::access_error;

use crate::interpreter::state::Value;

use super::state::ValueRef;

pub fn interpret_access(left: ValueRef, right: &str) -> ValueRef {
    match *left.borrow_mut() {
        Value::String(_) => string::access_string(left, right),
        Value::Integer(_) => integer::access_integer(left, right),
        Value::Float(_) => float::access_float(left, right),
        Value::Boolean(_) => boolean::access_boolean(left, right),
        Value::List(_) => list::access_list(left, right),
        Value::None => Rc::new(RefCell::new(access_error("None", right))),
        Value::Error(_) => left,
        Value::NativeFunction(_, _) => Rc::new(RefCell::new(Value::Error(
            "Functions do not have properties".to_string(),
        ))),
        Value::Function(_) => Rc::new(RefCell::new(Value::Error(
            "Functions do not have properties".to_string(),
        ))),
    }
}
