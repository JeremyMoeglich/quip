use crate::{
    ast::Expression,
    interpreter::state::{ProgramState, Value},
};

use super::interpret_expression;

pub fn interpret_call(
    (func, args): (&Expression, &Vec<Expression>),
    state: &ProgramState,
) -> Value {
    let mut arguments = Vec::new();
    for arg in args {
        arguments.push(interpret_expression(arg, state));
    }
    let function = interpret_expression(func, state);
    function.try_call(arguments)
}
