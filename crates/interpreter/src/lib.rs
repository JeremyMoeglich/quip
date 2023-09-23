mod access;
mod code_block;
mod expression;
mod native_functions;
pub mod state;
mod statement;


use self::{
    code_block::interpret_code_block,
    native_functions::NATIVE_FUNCTIONS,
    state::{program_state::ProgramState, value::value::Value, value_ref::ValueRef},
};

pub fn interpret_ast(block: CodeBlock) -> Result<ProgramState, String> {
    let mut state = ProgramState::new();
    let native_functions = NATIVE_FUNCTIONS
        .iter()
        .map(|(name, value)| (name.to_string(), value))
        .collect::<Vec<_>>();
    let res = interpret_code_block(
        &block,
        &state,
        native_functions
            .iter()
            .map(|(name, value)| (name, ValueRef::new((*value).clone())))
            .collect::<Vec<_>>(),
    );
    state = res.1;
    if let Value::Error(error) = &*res.0.clone().get() {
        Err(error.clone())
    } else {
        Ok(state)
    }
}
