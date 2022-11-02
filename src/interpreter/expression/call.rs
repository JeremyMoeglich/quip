use crate::{
    ast::Expression,
    interpreter::state::{ProgramState, Value},
};

use super::interpret_expression;

pub fn interpret_call((func, args): (Expression, Vec<Expression>), state: &mut ProgramState) -> Value {
    let mut arguments = Vec::new();
    for arg in args {
        arguments.push(interpret_expression(arg, state));
    }
    let function = interpret_expression(func, state);
    match function {
        Value::Function(function) => function.call(arguments, state),
        _ => panic!("Cannot call {:?} as a function", function),
    }
}
