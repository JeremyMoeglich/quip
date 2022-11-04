use crate::{
    ast::Expression,
    interpreter::state::{program_state::ProgramState, value_ref::ValueRef},
};

use super::interpret_expression;

pub fn interpret_get((value, index): (&Expression, &Expression), state: &ProgramState) -> ValueRef {
    let value = interpret_expression(value, state);
    let index = interpret_expression(index, state);
    value.try_get(&index)
}
