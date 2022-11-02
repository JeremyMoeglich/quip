use nom::{bytes::complete::tag, IResult};

use crate::ast::{Statement};

use super::{
    expression::parse_expression,
    identifier::parse_identifier,
    utils::{ws, Span},
};

pub fn parse_assignment(input: Span) -> IResult<Span, Statement> {
    let (input, name) = parse_identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = ws(input)?;
    let (input, expression) = parse_expression(input)?;
    Ok((
        input,
        Statement::Assignment(name, expression),
    ))
}
