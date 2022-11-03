use std::fmt::{Debug, Formatter};

use crate::ast::CodeBlock;

use super::{
    code_block::interpret_code_block,
    state::{ProgramState, Value},
};

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: CodeBlock,
    pub outer_state: ProgramState,
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // don't print state as it might be circular
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("parameters", &self.parameters)
            .field("body", &self.body)
            .finish()
    }
}

impl Function {
    pub fn call(&self, arguments: Vec<Value>) -> Value {
        if arguments.len() != self.parameters.len() {
            panic!("Argument and Parameter length should be the same")
        }
        interpret_code_block(
            &self.body,
            &self.outer_state,
            self.parameters.iter().zip(arguments).collect::<Vec<_>>(),
        )
        .0
    }
}
