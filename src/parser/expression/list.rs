use nom::{
    bytes::complete::tag,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{
    ast::Expression,
    parser::utils::{ws, Span},
};

use super::parse_expression;

pub fn parse_list(input: Span) -> IResult<Span, Expression> {
    let (input, value) = delimited(
        tuple((tag("["), ws)),
        separated_list0(tuple((ws, tag(","), ws)), parse_expression),
        tuple((ws, tag("]"))),
    )(input)?;
    Ok((input, Expression::List(value)))
}
