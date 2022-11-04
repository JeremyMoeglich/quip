use crate::{
    ast::{Expression, SingleOperation},
    interpreter::state::{program_state::ProgramState, value_ref::ValueRef},
};

use super::interpret_expression;

pub fn interpret_single_operation(
    (op, expr): (&SingleOperation, &Expression),
    state: &ProgramState,
) -> ValueRef {
    let expr = interpret_expression(&expr, state);
    expr.single_operation(op)
}
