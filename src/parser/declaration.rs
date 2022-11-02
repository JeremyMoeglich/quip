use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use crate::ast::Statement;

use super::{
    expression::parse_expression,
    identifier::parse_identifier,
    utils::{ws, ws1, Span},
};

pub fn parse_declaration(input: Span) -> IResult<Span, Statement> {
    let (input, _) = tag("let")(input)?;
    let (input, _) = ws1(input)?;
    let (input, mutable) = map(opt(tuple((tag("mut"), ws1))), |v| match v {
        Some(_) => true,
        None => false,
    })(input)?;
    let (input, identifier) = parse_identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, expression_opt) = map(opt(tuple((tag("="), ws, parse_expression))), |v| match v {
        Some((_, _, expression)) => Some(expression),
        None => None,
    })(input)?;
    Ok((
        input,
        Statement::Declaration((identifier, "some_empty_type".to_string()), mutable, expression_opt),
    ))
}
