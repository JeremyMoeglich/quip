use nom::{
    character::complete::char,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{
    fst::Expression,
    utils::{ws, Span},
};

use super::parse_expression;

pub fn parse_list(input: Span) -> IResult<Span, Expression> {
    let (input, value) = delimited(
        tuple((char('['), ws)),
        separated_list0(tuple((ws, char(','), ws)), parse_expression),
        tuple((ws, char(']'))),
    )(input)?;
    Ok((input, Expression::List(value)))
}
