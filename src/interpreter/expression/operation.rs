use crate::{
    ast::{Expression, Operator},
    interpreter::state::{program_state::ProgramState, value_ref::ValueRef},
};

use super::interpret_expression;

pub fn interpret_operation(
    (left, op, right): (&Box<Expression>, &Operator, &Box<Expression>),
    state: &ProgramState,
) -> ValueRef {
    let left = interpret_expression(&left, state);
    let right = interpret_expression(&right, state);

    left.operation(op, &right)
}
