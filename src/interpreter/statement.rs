use crate::ast::Statement;

use super::{
    expression::interpret_expression,
    function::Function,
    state::{ProgramState, Value},
};

pub fn interpret_statement(statement: Statement, state: &mut ProgramState) -> Value {
    match statement {
        Statement::Assignment(name, value) => {
            let value = interpret_expression(value, state);
            let mut state = state;
            state.variables.insert(name, value);
            Value::None
        }
        Statement::Function(name, parameters, body) => {
            let function = Function {
                name,
                parameters: parameters.iter().map(|(name, _)| name.clone()).collect(), // TODO: type annotations
                body,
            };
            let mut state = state;
            state
                .variables
                .insert(function.name.clone(), Value::Function(function));
            Value::None
        }
        Statement::Expression(expression) => {
            let value = interpret_expression(expression, state);
            value
        }
        _ => unimplemented!("Statement not implemented"),
    }
}
