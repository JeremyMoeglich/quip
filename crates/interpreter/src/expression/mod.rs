mod call;
mod get;
mod list;
mod literal;
mod operation;
mod single_operation;
mod string;
mod variable;

use parser::fst::Expression;

use self::{
    call::interpret_call, get::interpret_get, list::interpret_list, literal::interpret_literal,
    operation::interpret_operation, single_operation::interpret_single_operation,
    variable::interpret_variable,
};

use super::{
    code_block::interpret_code_block,
    state::{program_state::ProgramState, value_ref::ValueRef},
};

pub fn interpret_expression(expression: &Expression, state: &ProgramState) -> ValueRef {
    match expression {
        Expression::Literal(literal) => interpret_literal(literal, state),
        Expression::Variable(name) => interpret_variable(name, state),
        Expression::Call(func, args) => interpret_call((&func, &args), state),
        Expression::SingleOperation(op, expr) => interpret_single_operation((op, expr), state),
        Expression::Operation(left, op, right) => interpret_operation((&left, op, &right), state),
        Expression::List(elements) => interpret_list(elements, state),
        Expression::Get(value, index) => interpret_get((&value, &index), state),
        Expression::Block(statements) => interpret_code_block(statements, state, vec![]).0,
        Expression::Object(..) => todo!(),
    }
}
