mod assignment;
mod declaration;
mod enum_statement;
mod function;
mod generic;
mod if_statement;
mod impl_statement;
mod struct_statement;
mod type_statement;

use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use crate::parser::fst::Statement;

use self::{
    assignment::parse_assignment, declaration::parse_declaration, enum_statement::parse_enum,
    function::parse_function, if_statement::parse_if_statement, impl_statement::parse_impl,
    struct_statement::parse_struct, type_statement::parse_type_statement,
};

use super::{
    block::parse_block,
    expression::parse_expression,
    utils::{ws, Span},
};

pub fn parse_statement(input: Span) -> IResult<Span, Statement> {
    map(
        tuple((
            ws,
            alt((
                parse_function,
                parse_if_statement,
                parse_declaration,
                map(parse_block, |block| Statement::Scope(block)),
                parse_assignment,
                parse_struct,
                parse_impl,
                parse_enum,
                parse_type_statement,
                map(parse_expression, |expression| {
                    Statement::Expression(expression)
                }),
            )),
            ws,
            opt(char(';')),
        )),
        |(_, statement, _, semicolon)| match semicolon {
            Some(_) => Statement::StopReturn(Box::new(statement)),
            None => statement,
        },
    )(input)
}
