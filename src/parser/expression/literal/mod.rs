mod boolean;
mod float;
mod integer;
mod string;

use nom::{branch::alt, combinator::map, IResult};

use crate::parser::{fst::Literal, utils::Span};

pub fn parse_literal(input: Span) -> IResult<Span, Literal> {
    alt((
        map(string::parse_string_literal, Literal::String),
        map(float::parse_float, Literal::Float),
        map(integer::parse_integer, Literal::Integer),
        map(boolean::parse_boolean, Literal::Boolean),
    ))(input)
}
