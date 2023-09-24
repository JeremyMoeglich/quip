

use ast::{Operator, Expression};

use crate::state::{program_state::ProgramState, value_ref::ValueRef, value::value::Value};

use super::interpret_expression;

pub fn interpret_operation(
    (left, op, right): (&Box<Expression>, &Operator, &Box<Expression>),
    state: &ProgramState,
) -> ValueRef {
    let left = interpret_expression(&left, state);

    let right = match op {
        Operator::Access => {
            let name = match &**right {
                Expression::Variable(string) => string.clone(),
                _ => {
                    return ValueRef::new(Value::Error(
                        "Access operator can only be used with a literal string".to_string(),
                    ))
                }
            };
            ValueRef::new(Value::String(name))
        }
        _ => interpret_expression(&right, state),
    };
    left.operation(op, &right)
}
