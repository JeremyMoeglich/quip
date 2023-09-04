use crate::{
    ast::Expression,
    utils::{ws}, core::{Span, ParserResult, separated_list0, delimited},
};

use super::parse_expression;

pub fn parse_list<'a>(input: Span<'a>) -> ParserResult<'a, Expression, > {
    let (input, value) = delimited(
        tuple((char('['), ws)),
        separated_list0(tuple((ws, char(','), ws)), parse_expression),
        tuple((ws, char(']'))),
    )(input)?;
    Ok((input, Expression::List(value)))
}
