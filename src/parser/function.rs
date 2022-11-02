use nom::{
    bytes::complete::tag,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::ast::Statement;

use super::{
    block::parse_block,
    identifier::parse_identifier,
    utils::{ws, ws1, Span},
};

pub fn parse_function(input: Span) -> IResult<Span, Statement> {
    let (input, _) = tag("fn")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, params) = delimited(
        tuple((tag("("), ws)),
        separated_list0(tuple((ws, tag(","), ws)), parse_identifier),
        tuple((ws, tag(")"))),
    )(input)?; // TODO: type annotations
    let (input, _) = ws(input)?;
    let (input, code) = parse_block(input)?;
    let typed_params = params
        .iter()
        .map(|param| (param.clone(), "some_empty_type".to_string()))
        .collect();
    Ok((
        input,
        Statement::Function(name, typed_params, code),
    ))
}
