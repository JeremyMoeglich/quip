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
    expr.single_operation(op)
}
