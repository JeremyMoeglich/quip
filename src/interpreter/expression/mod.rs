mod call;
mod literal;
mod operation;
mod single_operation;
mod string;
mod variable;

use crate::ast::Expression;

use self::{
    call::interpret_call, literal::interpret_literal, operation::interpret_operation,
    single_operation::interpret_single_operation, variable::interpret_variable,
};

use super::state::{ProgramState, Value};

pub fn interpret_expression(expression: &Expression, state: &ProgramState) -> Value {
    match expression {
        Expression::Literal(literal) => interpret_literal(literal, state),
        Expression::Variable(name) => interpret_variable(name, state),
        Expression::Call(func, args) => interpret_call((&func, &args), state),
        Expression::SingleOperation(op, expr) => interpret_single_operation((op, expr), state),
        Expression::Operation(left, op, right) => interpret_operation((&left, op, &right), state),
    }
}
