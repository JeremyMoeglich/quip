use super::{
    state::{program_state::ProgramState, value_ref::ValueRef},
    statement::interpret_statement,
};
use parser::fst::CodeBlock;

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
        match value_ref {
            Some(value) => {
                return (value, state);
            }
            None => {}
        };
    }
    return (ValueRef::none(), state);
}
