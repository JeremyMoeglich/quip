use crate::{
    interpreter::state::{program_state::ProgramState, value_ref::ValueRef},
    parser::fst::Expression,
};

use super::interpret_expression;

pub fn interpret_call(
    (func, args): (&Expression, &Vec<Expression>),
    state: &ProgramState,
) -> ValueRef {
    let mut arguments = Vec::new();
    for arg in args {
        arguments.push(interpret_expression(arg, state));
    }
    let function = interpret_expression(func, state);
    function.try_call(arguments)
}
