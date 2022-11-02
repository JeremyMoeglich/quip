use crate::ast::CodeBlock;

use super::{state::{Value, ProgramState}, code_block::interpret_code_block};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: CodeBlock,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>, body: CodeBlock) -> Self {
        Self {
            name,
            parameters,
            body,
        }
    }

    pub fn call(&self, arguments: Vec<Value>, state: &mut ProgramState) -> Value {
        let mut state = state;
        for (parameter, argument) in self.parameters.iter().zip(arguments) {
            state.variables.insert(parameter.clone(), argument);
        }
        interpret_code_block(self.body.clone(), state)
    }
}