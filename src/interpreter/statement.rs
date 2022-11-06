use crate::parser::ast::{Expression, Statement};

use super::{
    expression::interpret_expression,
    state::{
        function::Function,
        program_state::ProgramState,
        value::{mutability::ValueMutability, value::Value},
        value_ref::ValueRef,
    },
};

pub fn interpret_statement(statement: &Statement, state: &ProgramState) -> Option<ValueRef> {
    match statement {
        Statement::StopReturn(statement) => match interpret_statement(statement, state) {
            Some(value) => match *value.resolve().get() {
                Value::Error(_) => Some(value),
                _ => None,
            },
            None => None,
        },
        Statement::Assignment(to_change, value) => {
            let value_ref = interpret_expression(value, state);
            if let Value::Error(_) = *value_ref.resolve().get() {
                return Some(value_ref);
            }
            match to_change {
                Expression::Variable(name) => match state.replace_variable(name, value_ref) {
                    Ok(_) => None,
                    Err(value) => Some(value),
                },
                _ => {
                    let to_change = interpret_expression(to_change, state);
                    if let Value::Error(_) = *to_change.resolve().get() {
                        return Some(to_change);
                    }
                    to_change.set(match value_ref.get().get_mutability() {
                        ValueMutability::Mutable => Value::Reference(value_ref.clone()),
                        ValueMutability::Immutable => value_ref.get().clone(),
                    });
                    None
                }
            }
        }
        Statement::Declaration((name, _), _, value) => {
            let value = match value {
                Some(value) => match interpret_expression(value, state) {
                    value_ref => match *value_ref.resolve().get() {
                        Value::Error(_) => return Some(value_ref),
                        _ => value_ref,
                    },
                },
                None => return None,
            };
            state.set_new_variable(name, value);
            None
        }
        Statement::Function(name, _generics, parameters, _return_type, body) => {
            let function = Function {
                name: name.clone(),
                parameters: parameters.iter().map(|(name, _)| name.clone()).collect(), // TODO: type annotations
                body: body.clone(),
                return_type: "TODO".to_string(),
                outer_state: state.clone(),
            };
            state.set_new_variable(name, ValueRef::new(Value::Function(function)));
            None
        }
        Statement::Expression(expression) => {
            let value = interpret_expression(expression, state);
            Some(value)
        }
        Statement::Scope(code_block) => {
            let (value, _) = super::code_block::interpret_code_block(code_block, state, vec![]);
            Some(value)
        }
        Statement::If(condition, if_block, else_block) => {
            let condition = interpret_expression(condition, state).resolve();
            if let Value::Error(_) = &*condition.clone().get() {
                return Some(condition);
            }
            if let Value::Boolean(condition) = &*condition.clone().get() {
                if *condition {
                    let (value, _) =
                        super::code_block::interpret_code_block(if_block, state, vec![]);
                    Some(value)
                } else {
                    let (value, _) =
                        super::code_block::interpret_code_block(else_block, state, vec![]);
                    Some(value)
                }
            } else {
                panic!("Condition should be a boolean");
            }
        }
        _ => unimplemented!("Statement not implemented {:?}", statement),
    }
}
