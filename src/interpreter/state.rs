use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    fmt::Display,
    rc::Rc,
};

use num::{BigInt, ToPrimitive};

use crate::ast::{Operator, SingleOperation};

use super::{access::interpret_access, function::Function};

#[derive(Debug, Clone)]
pub struct ValueRef {
    value: Rc<RefCell<Value>>,
}

impl ValueRef {
    pub fn new(value: Value) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
        }
    }

    pub fn set(&self, value: Value) {
        *self.value.borrow_mut() = value;
    }

    pub fn get(&self) -> Ref<Value> {
        self.value.borrow()
    }

    pub fn get_value(&self) -> Value {
        *self.get()
    }

    pub fn get_mut(&self) -> RefMut<Value> {
        self.value.borrow_mut()
    }

    pub fn get_property(&self, name: &str) -> ValueRef {
        interpret_access(self, name)
    }

    pub fn try_call(&self, args: Vec<ValueRef>) -> ValueRef {
        match self.get_value() {
            Value::Function(func) => func.call(args),
            Value::NativeFunction(context, func) => {
                func(context.clone().into_iter().chain(args).collect())
            }
            Value::Error(error) => Value::Error(error.clone()),
            _ => Value::Error("Only functions can be called".to_string()),
        }
    }

    pub fn operation(&self, op: &Operator, other: &ValueRef) -> ValueRef {
        if let Value::Error(error) = self.get_value() {
            return ValueRef::new(Value::Error(error.clone()));
        }
        if let Value::Error(error) = *other.get() {
            return ValueRef::new(Value::Error(error.clone()));
        }
        let err = || {
            ValueRef::new(Value::Error(format!(
                "Types {} and {} cannot be used with operator {:?}",
                self.get_value().get_type(),
                self.get_value().get_type(),
                op
            )))
        };
        match op {
            Operator::Add => match (self.get_value(), other.get_value()) {
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
            Operator::Subtract => match (self.get_value(), other.get_value()) {
                (Value::Integer(l), Value::Integer(r)) => ValueRef::new(Value::Integer(l - r)),
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Float(l - r)),
                _ => err(),
            },
            Operator::Multiply => match (self.get_value(), other.get_value()) {
                (Value::Integer(l), Value::Integer(r)) => ValueRef::new(Value::Integer(l * r)),
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Float(l * r)),
                _ => err(),
            },
            Operator::Divide => match (self.get_value(), other.get_value()) {
                (Value::Integer(l), Value::Integer(r)) => ValueRef::new(Value::Integer(l / r)),
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Float(l / r)),
                _ => err(),
            },
            Operator::Modulo => match (self.get_value(), other.get_value()) {
                (Value::Integer(l), Value::Integer(r)) => ValueRef::new(Value::Integer(l % r)),
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Float(l % r)),
                _ => err(),
            },
            Operator::Power => match (self.get_value(), other.get_value()) {
                (Value::Integer(l), Value::Integer(r)) => {
                    ValueRef::new(Value::Integer(l.pow(r.to_u32().unwrap())))
                }
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Float(l.powf(r))),
                _ => err(),
            },
            Operator::And => match (self.get_value(), other.get_value()) {
                (Value::Boolean(l), Value::Boolean(r)) => ValueRef::new(Value::Boolean(l && r)),
                _ => err(),
            },
            Operator::Or => match (self.get_value(), other.get_value()) {
                (Value::Boolean(l), Value::Boolean(r)) => ValueRef::new(Value::Boolean(l || r)),
                _ => err(),
            },
            Operator::Equals => match (self.get_value(), other.get_value()) {
                (Value::Integer(l), Value::Integer(r)) => ValueRef::new(Value::Boolean(l == r)),
                (Value::Float(l), Value::Float(r)) => ValueRef::new(Value::Boolean(l == r)),
                (Value::Boolean(l), Value::Boolean(r)) => ValueRef::new(Value::Boolean(l == r)),
                (Value::String(l), Value::String(r)) => ValueRef::new(Value::Boolean(l == r)),
                (Value::None, Value::None) => ValueRef::new(Value::Boolean(true)),
                _ => ValueRef::new(Value::Boolean(false)),
            },
            Operator::Access => match other.get_value() {
                Value::String(ref s) => self.get_property(s),
                _ => panic!("Invalid access: {:?} {:?}", self, other),
            },
            _ => unimplemented!("Operation {:?} is not implemented yet", op),
        }
    }

    pub fn single_operation(&self, op: &SingleOperation) -> ValueRef {
        match op {
            SingleOperation::Negate => match self.get_value() {
                Value::Integer(i) => ValueRef::new(Value::Integer(-i)),
                Value::Float(f) => ValueRef::new(Value::Float(-f)),
                _ => ValueRef::new(Value::Error("Cannot negate non-numeric value".to_string())),
            },
            SingleOperation::Not => match self.get_value() {
                Value::Boolean(b) => ValueRef::new(Value::Boolean(!b)),
                _ => ValueRef::new(Value::Error("Cannot negate non-boolean value".to_string())),
            },
            SingleOperation::Positive => match self.get_value() {
                Value::Integer(i) => self.clone(),
                Value::Float(f) => self.clone(),
                _ => ValueRef::new(Value::Error(
                    "Cannot unary add non-numeric value".to_string(),
                )),
            },
            SingleOperation::Panic => match self.get_value() {
                Value::None => panic!("Value is None"),
                _ => self.clone(),
            },
            _ => unimplemented!("Single operation {:?} is not implemented yet", op),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProgramScope {
    pub variables: HashMap<String, ValueRef>,
}

#[derive(Debug, Clone)]
pub struct ProgramState {
    pub scopes: Vec<Rc<RefCell<ProgramScope>>>,
}

impl ProgramState {
    pub fn new() -> Self {
        Self { scopes: vec![] }
    }
    pub fn run_function(&self, name: &str, args: Vec<Value>) -> ValueRef {
        let value = self.get_variable(name).expect("Function not found");
        match *value.get() {
            Value::Function(func) => func.call(args),
            _ => panic!("Only functions can be called"),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<ValueRef> {
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

    pub fn set_variable(&self, name: &str, value: ValueRef) {
        self.scopes
            .last()
            .unwrap()
            .borrow_mut()
            .variables
            .insert(name.to_string(), value);
    }

    pub fn reassign_variable(&self, name: &str, value: ValueRef) {
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
    NativeFunction(Vec<ValueRef>, fn(Vec<ValueRef>) -> ValueRef),
    Error(String),
    List(Vec<ValueRef>),
}

impl Value {
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
            Value::List(_) => "list",
        }
        .to_string()
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
            Value::Function(_) => write!(f, "Function"),
            Value::NativeFunction(_, _) => write!(f, "Native Function"),
            Value::Error(e) => write!(f, "Error: {}", e),
            Value::List(l) => write!(f, "{:?}", l),
        }
    }
}
