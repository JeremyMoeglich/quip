use num::ToPrimitive;

use crate::{
    ast::{Expression, Operator},
    interpreter::state::{ProgramState, Value},
};

use super::interpret_expression;

pub fn interpret_operation(
    (left, op, right): (Box<Expression>, Operator, Box<Expression>),
    state: &mut ProgramState,
) -> Value {
    let left = interpret_expression(*left, state);
    let right = interpret_expression(*right, state);

    match op {
        Operator::Add => match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l + r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l + r),
            (Value::String(l), Value::String(r)) => Value::String(l + &r),
            _ => panic!("Cannot add {:?} and {:?}", left, right),
        },
        Operator::Subtract => match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l - r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l - r),
            _ => panic!("Cannot subtract {:?} and {:?}", left, right),
        },
        Operator::Multiply => match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l * r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l * r),
            _ => panic!("Cannot multiply {:?} and {:?}", left, right),
        },
        Operator::Divide => match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l / r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l / r),
            _ => panic!("Cannot divide {:?} and {:?}", left, right),
        },
        Operator::Modulo => match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l % r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l % r),
            _ => panic!("Cannot modulo {:?} and {:?}", left, right),
        },
        Operator::Power => match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l.pow(r.to_u32().unwrap())),
            (Value::Float(l), Value::Float(r)) => Value::Float(l.powf(r)),
            _ => panic!("Cannot power {:?} and {:?}", left, right),
        },
        Operator::And => match (left, right) {
            (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(l && r),
            _ => panic!("Cannot and {:?} and {:?}", left, right),
        },
        Operator::Or => match (left, right) {
            (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(l || r),
            _ => panic!("Cannot or {:?} and {:?}", left, right),
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
