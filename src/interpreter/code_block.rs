use super::{
    state::{program_state::ProgramState, value::value::Value, value_ref::ValueRef},
    statement::interpret_statement,
};
use crate::ast::CodeBlock;

pub fn interpret_code_block(
    code_block: &CodeBlock,
    state: &ProgramState,
    added_variables: Vec<(&String, ValueRef)>,
) -> (ValueRef, ProgramState) {
    let state = state.new_scope();
    for (name, value) in added_variables {
        state.set_new_variable(&name, value);
    }
    for statement in code_block {
        let value_ref = interpret_statement(&statement, &state);
        let mut return_now = true;
        if let Value::Void = &*value_ref.get() {
            return_now = false;
        }
        if return_now {
            return (value_ref, state);
        }
    }
    return (ValueRef::none(), state);
}
