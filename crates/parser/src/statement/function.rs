

use crate::{
    ast::{Statement, TypeExpression},
    block::parse_block,
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{ws0, ws1, ws_delimited, Span},
};

use super::generic::parse_generics;

pub fn parse_function(input: Span) -> IResult<Span, Statement> {
    let (input, _) = tag("fn")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = ws0(input)?;
    let (input, generics) = parse_generics(input)?;
    let (input, _) = ws0(input)?;
    let (input, params) = delimited(
        tuple((char('('), ws0)),
        separated_list0(
            tuple((ws0, char(','), ws0)),
            tuple((
                parse_identifier,
                map(
                    opt(tuple((ws0, char(':'), ws0, parse_type_expression))),
                    |v| match v {
                        Some((_, _, _, type_)) => type_,
                        None => TypeExpression::Infer,
                    },
                ),
            )),
        ),
        tuple((ws0, char(')'))),
    )(input)?;
    let (input, _) = ws0(input)?;
    let (input, return_type) = map(
        opt(preceded(ws_delimited(tag("->")), parse_type_expression)),
        |v| match v {
            Some(type_) => type_,
            None => TypeExpression::Infer,
        },
    )(input)?;
    let (input, _) = ws0(input)?;
    let (input, code) = parse_block(input)?;
    Ok((
        input,
        Statement::Function(
            name,
            generics,
            params
                .iter()
                .map(|(name, type_)| (name.clone(), type_.clone()))
                .collect::<Vec<_>>(),
            return_type,
            code,
        ),
    ))
}
