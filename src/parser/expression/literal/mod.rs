mod boolean;
mod float;
mod integer;
mod string;

use nom::{branch::alt, combinator::map, IResult};

use crate::{parser::{utils::Span, ast::Literal}};

pub fn parse_literal(input: Span) -> IResult<Span, Literal> {
    alt((
        map(string::parse_string, Literal::String),
        map(float::parse_float, Literal::Float),
        map(integer::parse_integer, Literal::Integer),
        map(boolean::parse_boolean, Literal::Boolean),
    ))(input)
}
