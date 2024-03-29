use ast::CodeBlock;

use super::{
    state::{program_state::ProgramState, value_ref::ValueRef},
    statement::interpret_inner_statement,
};

pub fn interpret_code_block(
    code_block: &CodeBlock,
    state: &ProgramState,
    added_variables: Vec<(&String, ValueRef)>,
) -> (ValueRef, ProgramState) {
    let state = state.new_scope();
    for (name, value) in added_variables {
        state.set_new_variable(&name, value);
    }
    for statement in code_block.statements.clone() {
        let value_ref = interpret_inner_statement(&statement.inner, &state);
        match value_ref {
            Some(value) => {
                return (value, state);
            }
            None => {}
        };
    }
    return (ValueRef::none(), state);
}
