use crate::state::{program_state::ProgramState, value::value::Value, value_ref::ValueRef};
use ast::{Literal, Number};

pub fn interpret_literal(literal: &Literal, state: &ProgramState) -> ValueRef {
    match literal {
        Literal::Number(number) => match number {
            Number::Float(flt) => ValueRef::new(Value::Float(*flt)),
            Number::Integer(int) => ValueRef::new(Value::Integer(int.clone())),
        },
        Literal::String(string) => ValueRef::new(Value::String(string.clone())),
        Literal::Boolean(boolean) => ValueRef::new(Value::Boolean(*boolean)),
    }
}
