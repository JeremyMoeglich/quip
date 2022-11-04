use crate::ast::{Expression, Statement};

use super::{
    expression::interpret_expression,
    state::{
        function::Function,
        program_state::ProgramState,
        value::{mutability::ValueMutability, value::Value},
        value_ref::ValueRef,
    },
};

pub fn interpret_statement(statement: &Statement, state: &ProgramState) -> ValueRef {
    match statement {
        Statement::StopReturn(statement) => {
            let value = interpret_statement(statement, state);
            match *value.resolve().get() {
                Value::Error(_) => value,
                _ => ValueRef::none(),
            }
        }
        Statement::Assignment(to_change, value) => {
            let value_ref = interpret_expression(value, state);
            match to_change {
                Expression::Variable(name) => state.replace_variable(name, value_ref),
                _ => {
                    let to_change = interpret_expression(to_change, state);
                    to_change.set(match value_ref.get().get_mutability() {
                        ValueMutability::Mutable => Value::Reference(value_ref.clone()),
                        ValueMutability::Immutable => value_ref.get().clone(),
                    });
                }
            }
            ValueRef::none()
        }
        Statement::Declaration((name, _), _, value) => {
            let value = match value {
                Some(value) => interpret_expression(value, state),
                None => ValueRef::none(),
            };
            state.set_new_variable(name, value);
            ValueRef::none()
        }
        Statement::Function(name, parameters, body) => {
            let function = Function {
                name: name.clone(),
                parameters: parameters.iter().map(|(name, _)| name.clone()).collect(), // TODO: type annotations
                body: body.clone(),
                outer_state: state.clone(),
            };
            state.set_new_variable(name, ValueRef::new(Value::Function(function)));
            ValueRef::none()
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
            if let Value::Boolean(condition) = &*condition.clone().get() {
                if *condition {
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
