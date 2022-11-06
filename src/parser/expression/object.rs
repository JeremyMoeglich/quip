use nom::{
    character::complete::char, combinator::opt, multi::separated_list0, sequence::separated_pair,
    IResult,
};

use crate::parser::{
    ast::Expression,
    identifier::parse_identifier,
    utils::{ws, ws_delimited, Span},
};

use super::parse_expression;

pub fn parse_object(input: Span) -> IResult<Span, Expression> {
    let (input, name) = opt(parse_identifier)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('{')(input)?;
    let (input, _) = ws(input)?;
    let (input, values) = separated_list0(
        ws_delimited(char(',')),
        separated_pair(parse_identifier, ws_delimited(char(':')), parse_expression),
    )(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('}')(input)?;
    Ok((input, Expression::Object(name, values)))
}
