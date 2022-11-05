use crate::{
    ast::{Expression, Literal, Operator},
    interpreter::state::{program_state::ProgramState, value::value::Value, value_ref::ValueRef},
};

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
