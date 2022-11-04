use crate::{
    ast::Literal,
    interpreter::state::{program_state::ProgramState, value::value::Value, value_ref::ValueRef},
};

use super::string::interpret_string;

pub fn interpret_literal(literal: &Literal, state: &ProgramState) -> ValueRef {
    match literal {
        Literal::Integer(number) => ValueRef::new(Value::Integer(number.clone())),
        Literal::String(string) => interpret_string(string, state),
        Literal::Boolean(boolean) => ValueRef::new(Value::Boolean(*boolean)),
        Literal::None => ValueRef::none(),
        Literal::Float(number) => ValueRef::new(Value::Float(*number)),
    }
}
