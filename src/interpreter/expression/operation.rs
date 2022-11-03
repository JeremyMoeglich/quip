
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

    left.operation(op, &right)
}
