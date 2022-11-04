use std::fmt::Display;

use num::BigInt;

use crate::interpreter::state::{function::Function, value_ref::ValueRef};

use super::{mutability::ValueMutability, types::ValueType};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(BigInt),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
    Void,
    Function(Function),
    NativeFunction(Vec<ValueRef>, fn(Vec<ValueRef>) -> ValueRef),
    Error(String),
    List(Vec<ValueRef>),
    Reference(ValueRef),
    //Object(HashMap<ValueRef, ValueRef>),
}

impl Value {
    pub fn get_type(&self) -> ValueType {
        match self {
            Value::Integer(_) => ValueType::Integer,
            Value::Float(_) => ValueType::Float,
            Value::String(_) => ValueType::String,
            Value::Boolean(_) => ValueType::Boolean,
            Value::None => ValueType::None,
            Value::Void => {
                unreachable!("Void is not a value, so it should not be possible to get its type")
            }
            Value::Function(_) => ValueType::Function,
            Value::NativeFunction(..) => ValueType::NativeFunction,
            Value::Error(_) => ValueType::Error,
            Value::List(_) => ValueType::List,
            Value::Reference(_) => ValueType::Reference,
            //Value::Object(_) => ValueType::Object,
        }
    }

    pub fn get_mutability(&self) -> ValueMutability {
        match self {
            Value::Integer(_) => ValueMutability::Immutable,
            Value::Float(_) => ValueMutability::Immutable,
            Value::String(_) => ValueMutability::Immutable,
            Value::Boolean(_) => ValueMutability::Immutable,
            Value::None => ValueMutability::Immutable,
            Value::Void => {
                unreachable!(
                    "Void is not a value, so it should not be possible to get its mutability"
                )
            }
            Value::Function(_) => ValueMutability::Immutable,
            Value::NativeFunction(..) => ValueMutability::Immutable,
            Value::Error(_) => ValueMutability::Immutable,
            Value::List(_) => ValueMutability::Mutable,
            Value::Reference(_) => ValueMutability::Immutable,
            //Value::Object(_) => ValueMutability::Mutable,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(flt) => write!(f, "{}", flt),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::None => write!(f, "None"),
            Value::Void => {
                unreachable!("Void is not a value, so it should not be possible to display it")
            }
            Value::Function(_) => write!(f, "Function"),
            Value::NativeFunction(_, _) => write!(f, "Native Function"),
            Value::Error(e) => write!(f, "Error: {}", e),
            Value::List(l) => write!(f, "{:?}", l),
            Value::Reference(r) => write!(f, "{}", r.get()),
            //Value::Object(o) => write!(f, "{:?}", o),
        }
    }
}
