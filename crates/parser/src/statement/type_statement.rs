use nom::{bytes::complete::tag, character::complete::char, IResult};

use crate::{
    ast::Statement,
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{ws, ws1, Span},
};

use super::generic::parse_generics;

pub fn parse_type_statement(input: Span) -> IResult<Span, Statement> {
    let (input, _) = tag("type")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, generics) = parse_generics(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = ws(input)?;
    let (input, type_) = parse_type_expression(input)?;
    Ok((input, Statement::TypeAlias(name, generics, type_)))
}
