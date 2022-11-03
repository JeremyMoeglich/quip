use crate::{
    ast::{FancyString, FancyStringFragment},
    interpreter::state::{ProgramState, Value},
};

use super::interpret_expression;

pub fn interpret_string(string: &FancyString, state: &ProgramState) -> Value {
    match state.get_variable("join") {
        Some(value) => value.try_call(
            string
                .iter()
                .map(|string_part| match string_part {
                    FancyStringFragment::LiteralString(literal) => Value::String(literal.clone()),
                    FancyStringFragment::Expression(expression) => {
                        interpret_expression(expression, state)
                    }
                    FancyStringFragment::FormatPlaceholder => {
                        unimplemented!("Format placeholders are not implemented yet")
                    }
                })
                .collect(),
        ),
        _ => Value::Error("join() is not defined".to_string()),
    }
}
