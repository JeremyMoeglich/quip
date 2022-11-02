mod code_block;
mod state;
mod statement;
mod function;
mod expression;

use crate::ast::{CodeBlock};

use self::{state::ProgramState, code_block::interpret_code_block};

pub fn interpret_ast(block: CodeBlock) {
    let state = ProgramState::new();
    let state = interpret_code_block(block, &mut state);
    println!("Final state: {:#?}", state);
}