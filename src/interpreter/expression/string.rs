use crate::{interpreter::state::{
    program_state::ProgramState, value::value::Value, value_ref::ValueRef,
}, parser::ast::{FancyString, FancyStringFragment}};

use super::interpret_expression;

pub fn interpret_string(string: &FancyString, state: &ProgramState) -> ValueRef {
    match state.get_variable("join") {
        Some(value) => value.try_call(
            string
                .iter()
                .map(|string_part| match string_part {
                    FancyStringFragment::LiteralString(literal) => {
                        ValueRef::new(Value::String(literal.clone()))
                    }
                    FancyStringFragment::Expression(expression) => {
                        interpret_expression(expression, state)
                    }
                    FancyStringFragment::FormatPlaceholder => {
                        unimplemented!("Format placeholders are not implemented yet")
                    }
                })
                .collect(),
        ),
        _ => ValueRef::new(Value::Error("join() is not defined".to_string())),
    }
}
