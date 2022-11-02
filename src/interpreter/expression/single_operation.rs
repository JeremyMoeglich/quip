use std::{rc::Rc, cell::RefCell};

use crate::{
    ast::{Expression, SingleOperation},
    interpreter::state::{ProgramState, Value},
};

use super::interpret_expression;

pub fn interpret_single_operation(
    (op, expr): (&SingleOperation, &Expression),
    state: &ProgramState,
) -> Value {
    let expr = interpret_expression(&expr, state);
    match op {
        SingleOperation::Negate => match expr {
            Value::Integer(i) => Value::Integer(-i),
            Value::Float(f) => Value::Float(-f),
            _ => panic!("Cannot negate {:?}", expr),
        },
        SingleOperation::Not => match expr {
            Value::Boolean(b) => Value::Boolean(!b),
            _ => panic!("Cannot negate {:?}", expr),
        },
        SingleOperation::Positive => match expr {
            Value::Integer(i) => Value::Integer(i),
            Value::Float(f) => Value::Float(f),
            _ => panic!("Not a number {:?}", expr),
        },
        SingleOperation::Panic => match expr {
            Value::None => panic!("Value is None"),
            _ => expr,
        },
        _ => unimplemented!("Single operation {:?} is not implemented yet", op),
    }
}
