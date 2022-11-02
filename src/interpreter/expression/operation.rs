use std::{rc::Rc, cell::RefCell};

use num::ToPrimitive;

use crate::{
    ast::{Expression, Operator},
    interpreter::state::{ProgramState, Value},
};

use super::interpret_expression;

pub fn interpret_operation(
    (left, op, right): (&Box<Expression>, &Operator, &Box<Expression>),
    state: &ProgramState,
) -> Value {
    let left = interpret_expression(&left, state);
    let right = interpret_expression(&right, state);

    let err = || format!("Invalid operation: {:?} {:?} {:?}", left, op, right);

    match op {
        Operator::Add => match (&left, &right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l + r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l + r),
            (Value::String(l), Value::String(r)) => Value::String(l.clone() + r),
            _ => panic!("{}", err()),
        },
        Operator::Subtract => match (&left, &right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l - r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l - r),
            _ => panic!("{}", err()),
        },
        Operator::Multiply => match (&left, &right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l * r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l * r),
            _ => panic!("{}", err()),
        },
        Operator::Divide => match (&left, &right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l / r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l / r),
            _ => panic!("{}", err()),
        },
        Operator::Modulo => match (&left, &right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l % r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l % r),
            _ => panic!("{}", err()),
        },
        Operator::Power => match (&left, &right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l.pow(r.to_u32().unwrap())),
            (Value::Float(l), Value::Float(r)) => Value::Float(l.powf(*r)),
            _ => panic!("{}", err()),
        },
        Operator::And => match (&left, &right) {
            (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(*l && *r),
            _ => panic!("{}", err()),
        },
        Operator::Or => match (&left, &right) {
            (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(*l || *r),
            _ => panic!("{}", err()),
        },
        Operator::Equals => match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Boolean(l == r),
            (Value::Float(l), Value::Float(r)) => Value::Boolean(l == r),
            (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(l == r),
            (Value::String(l), Value::String(r)) => Value::Boolean(l == r),
            (Value::None, Value::None) => Value::Boolean(true),
            _ => Value::Boolean(false),
        },
        _ => unimplemented!("Operation {:?} is not implemented yet", op),
    }
}
