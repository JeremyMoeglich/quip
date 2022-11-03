mod access_error;
mod boolean;
mod float;
mod integer;
mod string;

use access_error::access_error;

use crate::interpreter::state::Value;

pub fn interpret_access(left: &Value, right: &str) -> Value {
    match left {
        Value::String(ref s) => string::access_string(s.clone(), right),
        Value::Integer(ref i) => integer::access_integer(i.to_owned(), right),
        Value::Float(ref f) => float::access_float(*f, right),
        Value::Boolean(ref b) => boolean::access_boolean(*b, right),
        Value::None => access_error("None", right),
        Value::Error(ref e) => Value::Error(e.clone()),
        Value::NativeFunction(_, _) => Value::Error("Functions do not have properties".to_string()),
        Value::Function(_) => Value::Error("Functions do not have properties".to_string()),
    }
}
