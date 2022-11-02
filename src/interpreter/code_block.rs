use crate::ast::CodeBlock;
use super::{state::{ProgramState, Value}, statement::interpret_statement};

pub fn interpret_code_block(code_block: CodeBlock, state: &mut ProgramState) -> Value {
    let mut state = state;
    let mut return_value = Value::None;
    for statement in code_block {
        return_value = interpret_statement(statement, state);
    }
    return_value
}