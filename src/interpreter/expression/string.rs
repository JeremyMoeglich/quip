use crate::{
    ast::{FancyString, FancyStringFragment},
    interpreter::state::{ProgramState, Value},
};

use super::interpret_expression;

pub fn interpret_string(string: FancyString, state: &mut ProgramState) -> Value {
    Value::String(
        string
            .iter()
            .map(|string_part| match string_part {
                FancyStringFragment::LiteralString(literal) => *literal,
                FancyStringFragment::Expression(expression) => {
                    let value = interpret_expression(*expression, &mut state);
                    match value {
                        Value::String(string) => string,
                        _ => panic!("Cannot convert {:?} to string", value),
                    }
                }
                FancyStringFragment::FormatPlaceholder => {
                    unimplemented!("Format placeholders are not implemented yet")
                }
            })
            .collect::<String>(),
    )
}
