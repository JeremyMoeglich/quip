use nom::{character::complete::digit1, IResult};
use num::BigInt;

use crate::parser::utils::Span;

pub fn parse_integer(input: Span) -> IResult<Span, BigInt> {
    let (input, digits) = digit1(input)?;
    let mut value = digits.fragment().to_string();
    Ok((
        input,
        BigInt::parse_bytes(value.as_bytes(), 10).expect("Failed to parse integer, parser bug"),
    ))
}
