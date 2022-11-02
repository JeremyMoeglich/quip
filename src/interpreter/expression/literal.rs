use crate::{
    ast::Literal,
    interpreter::state::{ProgramState, Value},
};

use super::string::interpret_string;

pub fn interpret_literal(literal: Literal, state: &mut ProgramState) -> Value {
    match literal {
        Literal::Integer(number) => Value::Integer(number),
        Literal::String(string) => interpret_string(string, state),
        Literal::Boolean(boolean) => Value::Boolean(boolean),
        Literal::None => Value::None,
        Literal::Float(number) => Value::Float(number),
    }
}
