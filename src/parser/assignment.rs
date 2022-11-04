use nom::{bytes::complete::tag, IResult};

use crate::ast::Statement;

use super::{
    expression::parse_expression,
    utils::{ws, Span},
};

pub fn parse_assignment(input: Span) -> IResult<Span, Statement> {
    let (input, to_change) = parse_expression(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = ws(input)?;
    let (input, expression) = parse_expression(input)?;
    Ok((input, Statement::Assignment(to_change, expression)))
}
