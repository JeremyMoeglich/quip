use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

use crate::parser::utils::Span;

pub fn parse_boolean(input: Span) -> IResult<Span, bool> {
    let (input, value) = alt((value(true, tag("true")), value(false, tag("false"))))(input)?;
    Ok((input, value))
}
