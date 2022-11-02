use nom::{branch::alt, combinator::{map, opt}, sequence::tuple, IResult, bytes::complete::tag};

use crate::ast::Statement;

use super::{
    assignment::parse_assignment, block::parse_block, declaration::parse_declaration,
    expression::parse_expression, function::parse_function, if_statement::parse_if_statement,
    utils::{Span, ws},
};

pub fn parse_statement(input: Span) -> IResult<Span, Statement> {
    map(
        tuple((
            ws,
            alt((
                parse_function,
                parse_if_statement,
                parse_declaration,
                parse_assignment,
                map(parse_block, |block| Statement::Scope(block)),
                map(parse_expression, |expression| {
                    Statement::Expression(expression)
                }),
            )),
            ws,
            opt(tag(";")),
        )),
        |(_, statement, _, _)| statement,
    )(input)
}
