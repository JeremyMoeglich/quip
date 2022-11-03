use crate::ast::Statement;

use super::{
    expression::interpret_expression,
    function::Function,
    state::{ProgramState, Value},
};

pub fn interpret_statement(statement: &Statement, state: &ProgramState) -> ValueRef {
    match statement {
        Statement::Assignment(name, value) => {
            let value = interpret_expression(value, state);
            state.reassign_variable(name, value);
            Value::None
        }
        Statement::Declaration((name, _), _, value) => {
            let value = match value {
                Some(value) => interpret_expression(value, state),
                None => Value::None,
            };
            state.set_variable(name, value);
            Value::None
        }
        Statement::Function(name, parameters, body) => {
            let function = Function {
                name: name.clone(),
                parameters: parameters.iter().map(|(name, _)| name.clone()).collect(), // TODO: type annotations
                body: body.clone(),
                outer_state: state.clone(),
            };
            state.set_variable(name, Value::Function(function));
            Value::None
        }
        Statement::Expression(expression) => {
            let value = interpret_expression(expression, state);
            value
        }
        Statement::Scope(code_block) => {
            let (value, _) = super::code_block::interpret_code_block(code_block, state, vec![]);
            value
        }
        Statement::If(condition, if_block, else_block) => {
            let condition = interpret_expression(condition, state);
            if let Value::Boolean(condition) = condition {
                if condition {
                    let (value, _) =
                        super::code_block::interpret_code_block(if_block, state, vec![]);
                    value
                } else {
                    let (value, _) =
                        super::code_block::interpret_code_block(else_block, state, vec![]);
                    value
                }
            } else {
                panic!("Condition should be a boolean");
            }
        }
        _ => unimplemented!("Statement not implemented {:?}", statement),
    }
}
