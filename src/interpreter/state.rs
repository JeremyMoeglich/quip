use std::{cell::RefCell, collections::HashMap, rc::Rc};

use num::{BigInt, ToPrimitive};

use crate::ast::{Operator, SingleOperation};

use super::{access::interpret_access, function::Function};

#[derive(Debug, Clone)]
pub struct ProgramScope {
    pub variables: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ProgramState {
    pub scopes: Vec<Rc<RefCell<ProgramScope>>>,
}

impl ProgramState {
    pub fn new() -> Self {
        Self { scopes: vec![] }
    }
    pub fn run_function(&self, name: &str, args: Vec<Value>) -> Value {
        let value = self.get_variable(name).expect("Function not found");
        match value {
            Value::Function(func) => func.call(args),
            _ => panic!("Only functions can be called"),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.borrow().variables.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn new_scope(&self) -> Self {
        let mut new_state = self.clone();
        new_state.scopes.push(Rc::new(RefCell::new(ProgramScope {
            variables: HashMap::new(),
        })));
        new_state
    }

    pub fn set_variable(&self, name: &str, value: Value) {
        self.scopes
            .last()
            .unwrap()
            .borrow_mut()
            .variables
            .insert(name.to_string(), value);
    }

    pub fn reassign_variable(&self, name: &str, value: Value) {
        for scope in self.scopes.iter().rev() {
            if scope.borrow().variables.contains_key(name) {
                scope.borrow_mut().variables.insert(name.to_string(), value);
                return;
            }
        }
        panic!("Variable does not exist");
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(BigInt),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
    Function(Function),
    NativeFunction(Vec<Value>, fn(Vec<Value>) -> Value),
    Error(String),
}

impl Value {
    pub fn access_property(&self, name: &str) -> Value {
        interpret_access(&self, &name)
    }

    fn get_type(&self) -> String {
        match self {
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::None => "none",
            Value::Function(_) => "function",
            Value::NativeFunction(_, _) => "native function",
            Value::Error(_) => "error",
        }
        .to_string()
    }

    pub fn try_call(&self, args: Vec<Value>) -> Value {
        match self {
            Value::Function(func) => func.call(args),
            Value::NativeFunction(context, func) => {
                func(context.clone().into_iter().chain(args).collect())
            }
            Value::Error(error) => Value::Error(error.clone()),
            _ => Value::Error("Only functions can be called".to_string()),
        }
    }

    pub fn operation(&self, op: &Operator, other: &Value) -> Value {
        if let Value::Error(error) = self {
            return Value::Error(error.clone());
        }
        if let Value::Error(error) = other {
            return Value::Error(error.clone());
        }
        let err = || {
            format!(
                "Types {} and {} cannot be used with operator {:?}",
                self.get_type(),
                other.get_type(),
                op
            )
        };
        match op {
            Operator::Add => match (self, other) {
                (Value::Integer(l), Value::Integer(r)) => Value::Integer(l + r),
                (Value::Float(l), Value::Float(r)) => Value::Float(l + r),
                (Value::Float(l), Value::Integer(r)) => Value::Float(l + r.to_f64().unwrap()),
                (Value::Integer(l), Value::Float(r)) => Value::Float(l.to_f64().unwrap() + r),
                (Value::String(l), Value::String(r)) => Value::String(l.clone() + r),
                _ => Value::Error(err()),
            },
            Operator::Subtract => match (self, other) {
                (Value::Integer(l), Value::Integer(r)) => Value::Integer(l - r),
                (Value::Float(l), Value::Float(r)) => Value::Float(l - r),
                _ => Value::Error(err()),
            },
            Operator::Multiply => match (self, other) {
                (Value::Integer(l), Value::Integer(r)) => Value::Integer(l * r),
                (Value::Float(l), Value::Float(r)) => Value::Float(l * r),
                _ => Value::Error(err()),
            },
            Operator::Divide => match (self, other) {
                (Value::Integer(l), Value::Integer(r)) => Value::Integer(l / r),
                (Value::Float(l), Value::Float(r)) => Value::Float(l / r),
                _ => Value::Error(err()),
            },
            Operator::Modulo => match (self, other) {
                (Value::Integer(l), Value::Integer(r)) => Value::Integer(l % r),
                (Value::Float(l), Value::Float(r)) => Value::Float(l % r),
                _ => Value::Error(err()),
            },
            Operator::Power => match (self, other) {
                (Value::Integer(l), Value::Integer(r)) => {
                    Value::Integer(l.pow(r.to_u32().unwrap()))
                }
                (Value::Float(l), Value::Float(r)) => Value::Float(l.powf(*r)),
                _ => Value::Error(err()),
            },
            Operator::And => match (self, other) {
                (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(*l && *r),
                _ => Value::Error(err()),
            },
            Operator::Or => match (self, other) {
                (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(*l || *r),
                _ => Value::Error(err()),
            },
            Operator::Equals => match (self, other) {
                (Value::Integer(l), Value::Integer(r)) => Value::Boolean(l == r),
                (Value::Float(l), Value::Float(r)) => Value::Boolean(l == r),
                (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(l == r),
                (Value::String(l), Value::String(r)) => Value::Boolean(l == r),
                (Value::None, Value::None) => Value::Boolean(true),
                _ => Value::Boolean(false),
            },
            Operator::Access => match other {
                Value::String(ref s) => self.access_property(s),
                _ => panic!("Invalid access: {:?} {:?}", self, other),
            },
            _ => unimplemented!("Operation {:?} is not implemented yet", op),
        }
    }

    pub fn single_operation(&self, op: &SingleOperation) -> Value {
        match op {
            SingleOperation::Negate => match self {
                Value::Integer(i) => Value::Integer(-i),
                Value::Float(f) => Value::Float(-f),
                _ => panic!("Cannot negate {:?}", self),
            },
            SingleOperation::Not => match self {
                Value::Boolean(b) => Value::Boolean(!b),
                _ => panic!("Cannot negate {:?}", self),
            },
            SingleOperation::Positive => match self {
                Value::Integer(i) => Value::Integer(i.clone()),
                Value::Float(f) => Value::Float(*f),
                _ => panic!("Not a number {:?}", self),
            },
            SingleOperation::Panic => match self {
                Value::None => panic!("Value is None"),
                _ => self.clone(),
            },
            _ => unimplemented!("Single operation {:?} is not implemented yet", op),
        }
    }
}
