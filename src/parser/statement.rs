use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use crate::ast::Statement;

use super::{
    assignment::parse_assignment,
    block::parse_block,
    declaration::parse_declaration,
    expression::parse_expression,
    function::parse_function,
    if_statement::parse_if_statement,
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
                map(parse_expression, |expression| {
                    Statement::Expression(expression)
                }),
            )),
            ws,
            opt(tag(";")),
        )),
        |(_, statement, _, semicolon)| match semicolon {
            Some(_) => Statement::StopReturn(Box::new(statement)),
            None => statement,
        },
    )(input)
}
