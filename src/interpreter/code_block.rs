use std::{rc::Rc, cell::RefCell};

use super::{
    state::{ProgramState, Value},
    statement::interpret_statement,
};
use crate::ast::CodeBlock;

pub fn interpret_code_block(
    code_block: &CodeBlock,
    state: &ProgramState,
) -> (Value, ProgramState) {
    let state = state.new_scope();
    let mut return_value = Value::None;
    for statement in code_block {
        return_value = interpret_statement(&statement, state);
    }
    return (return_value, state);
}
