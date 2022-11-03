mod access;
mod code_block;
mod expression;
mod function;
mod native_functions;
pub mod state;
mod statement;

use crate::ast::CodeBlock;

use self::{
    code_block::interpret_code_block,
    native_functions::NATIVE_FUNCTIONS,
    state::{ProgramState, Value},
};

pub fn interpret_ast(block: CodeBlock) -> ProgramState {
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
            .map(|(name, value)| (name, (*value).clone()))
            .collect::<Vec<_>>(),
    );
    state = res.1;
    if let Value::Error(error) = res.0 {
        println!("Program exited with an error: ");
        println!("{}", error);
    }
    state
}
