use super::{
    state::{ProgramState, Value},
    statement::interpret_statement,
};
use crate::ast::CodeBlock;

pub fn interpret_code_block(
    code_block: &CodeBlock,
    state: &ProgramState,
    added_variables: Vec<(&String, Value)>,
) -> (Value, ProgramState) {
    let state = state.new_scope();
    for (name, value) in added_variables {
        state.set_variable(&name, value);
    }
    let mut return_value = Value::None;
    for statement in code_block {
        return_value = interpret_statement(&statement, &state);
    }
    return (return_value, state);
}
