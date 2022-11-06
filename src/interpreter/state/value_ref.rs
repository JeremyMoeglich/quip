use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use num::ToPrimitive;

use crate::{
    interpreter::access::interpret_access,
    parser::ast::{Operator, SingleOperation},
};

use super::value::value::Value;

#[derive(Debug, Clone)]
pub struct ValueRef {
    pub value: Rc<RefCell<Value>>,
}

impl ValueRef {
    pub fn new(value: Value) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
        }
    }

    pub fn none() -> Self {
        Self::new(Value::None)
    }

    pub fn set(&self, value: Value) {
        *self.value.borrow_mut() = value;
    }

    pub fn get(&self) -> Ref<Value> {
        let value = self.value.borrow();
        value
    }

    pub fn resolve(&self) -> ValueRef {
        match &*self.get() {
            Value::Reference(reference) => reference.resolve(),
            _ => self.clone(),
        }
    }

    pub fn get_property(&self, name: &str) -> ValueRef {
        interpret_access(self.clone(), name)
    }

    pub fn try_call(&self, args: Vec<ValueRef>) -> ValueRef {
        match &*self.resolve().get() {
            Value::Function(func) => func.call(args),
            Value::NativeFunction(context, func) => {
                func(context.clone().into_iter().chain(args).collect())
            }
            Value::Error(_) => self.clone(),
            _ => ValueRef::new(Value::Error("Only functions can be called".to_string())),
        }
    }

    pub fn operation(&self, op: &Operator, other: &ValueRef) -> ValueRef {
        let left = self.resolve();
        let right = other.resolve();
        if let Value::Error(error) = &*left.get() {
            return ValueRef::new(Value::Error(error.clone()));
        }
        if let Value::Error(error) = &*right.get() {
            return ValueRef::new(Value::Error(error.clone()));
        }
        let err = || {
            ValueRef::new(Value::Error(format!(
                "Types {} and {} cannot be used with operator {:?}",
                (*left.get()).get_type(),
                (*left.get()).get_type(),
                op
            )))
        };
        match op {
            Operator::Add => match (&*left.get(), &*right.get()) {
                (Value::Integer(l), Value::Integer(r)) => ValueRef::new(Value::Integer(l + r)),
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Float(l + r)),
                (Value::Float(l), Value::Integer(r)) => {
                    ValueRef::new(Value::Float(l + r.to_f64().unwrap()))
                }
                (Value::Integer(l), Value::Float(r)) => {
                    ValueRef::new(Value::Float(l.to_f64().unwrap() + r))
                }
                (Value::String(l), Value::String(r)) => {
                    ValueRef::new(Value::String(l.clone() + &r))
                }
                _ => err(),
            },
            Operator::Subtract => match (&*left.get(), &*right.get()) {
                (Value::Integer(l), Value::Integer(r)) => ValueRef::new(Value::Integer(l - r)),
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Float(l - r)),
                _ => err(),
            },
            Operator::Multiply => match (&*left.get(), &*right.get()) {
                (Value::Integer(l), Value::Integer(r)) => ValueRef::new(Value::Integer(l * r)),
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Float(l * r)),
                _ => err(),
            },
            Operator::Divide => match (&*left.get(), &*right.get()) {
                (Value::Integer(l), Value::Integer(r)) => ValueRef::new(Value::Integer(l / r)),
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Float(l / r)),
                _ => err(),
            },
            Operator::Modulo => match (&*left.get(), &*right.get()) {
                (Value::Integer(l), Value::Integer(r)) => ValueRef::new(Value::Integer(l % r)),
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Float(l % r)),
                _ => err(),
            },
            Operator::Power => match (&*left.get(), &*right.get()) {
                (Value::Integer(l), Value::Integer(r)) => {
                    ValueRef::new(Value::Integer(l.pow(r.to_u32().unwrap())))
                }
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Float(l.powf(*r))),
                _ => err(),
            },
            Operator::And => match (&*left.get(), &*right.get()) {
                (Value::Boolean(l), Value::Boolean(r)) => ValueRef::new(Value::Boolean(*l && *r)),
                _ => err(),
            },
            Operator::Or => match (&*left.get(), &*right.get()) {
                (Value::Boolean(l), Value::Boolean(r)) => ValueRef::new(Value::Boolean(*l || *r)),
                _ => err(),
            },
            Operator::Equals => match (&*left.get(), &*right.get()) {
                (Value::Integer(l), Value::Integer(r)) => ValueRef::new(Value::Boolean(l == r)),
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Boolean(l == r)),
                (Value::Boolean(l), Value::Boolean(r)) => ValueRef::new(Value::Boolean(l == r)),
                (Value::String(l), Value::String(r)) => ValueRef::new(Value::Boolean(l == r)),
                (Value::None, Value::None) => ValueRef::new(Value::Boolean(true)),
                _ => ValueRef::new(Value::Boolean(false)),
            },
            Operator::Access => match *right.get() {
                Value::String(ref s) => left.get_property(s),
                _ => ValueRef::new(Value::Error("Cannot access with non-string".to_string())),
            },
            _ => unimplemented!("Operation {:?} is not implemented yet", op),
        }
    }

    pub fn single_operation(&self, op: &SingleOperation) -> ValueRef {
        let value = self.resolve();
        match op {
            SingleOperation::Negate => match &*value.get() {
                Value::Integer(i) => ValueRef::new(Value::Integer(-i)),
                Value::Float(f) => ValueRef::new(Value::Float(-f)),
                _ => ValueRef::new(Value::Error("Cannot negate non-numeric value".to_string())),
            },
            SingleOperation::Not => match *value.get() {
                Value::Boolean(b) => ValueRef::new(Value::Boolean(!b)),
                _ => ValueRef::new(Value::Error("Cannot negate non-boolean value".to_string())),
            },
            SingleOperation::Positive => match &*value.get() {
                Value::Integer(_) => value.clone(),
                Value::Float(_) => value.clone(),
                _ => ValueRef::new(Value::Error(
                    "Cannot unary add non-numeric value".to_string(),
                )),
            },
            SingleOperation::Panic => match *value.get() {
                Value::None => panic!("Value is None"),
                _ => value.clone(),
            },
            _ => unimplemented!("Single operation {:?} is not implemented yet", op),
        }
    }

    pub fn try_get(&self, index: &ValueRef) -> ValueRef {
        let value = self.resolve();
        let index = index.resolve();
        match (&*value.clone().get(), &*index.clone().get()) {
            (Value::List(l), Value::Integer(i)) => l[i.to_usize().unwrap()].clone(),
            (Value::String(s), Value::Integer(i)) => ValueRef::new(Value::String(
                s.chars().nth(i.to_usize().unwrap()).unwrap().to_string(),
            )),
            _ => ValueRef::new(Value::Error(format!(
                "Cannot get {:?} from {:?}",
                value.get().get_type(),
                index.get().get_type()
            ))),
        }
    }
}
