mod code_block;
mod expression;
mod function;
pub mod state;
mod statement;

use std::{cell::RefCell, rc::Rc};

use crate::ast::CodeBlock;

use self::{code_block::interpret_code_block, state::ProgramState};

pub fn interpret_ast(block: CodeBlock) -> Rc<RefCell<ProgramState>> {
    let mut state = Rc::new(RefCell::new(ProgramState::new()));
    state = interpret_code_block(&block, state).1;
    state
}
