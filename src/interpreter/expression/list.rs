use crate::{
    ast::Expression,
    interpreter::state::{program_state::ProgramState, value::value::Value, value_ref::ValueRef},
};

use super::interpret_expression;

pub fn interpret_list(elements: &Vec<Expression>, state: &ProgramState) -> ValueRef {
    let mut values = Vec::new();
    for element in elements {
        values.push(interpret_expression(element, state));
    }
    ValueRef::new(Value::List(values))
}
