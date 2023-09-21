use ast::{Statement, TypeExpression};
use parser_core::*;
use crate::{
    block::parse_block,
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{opt, ws0, ws1, ws_delimited},
};

use super::generic::parse_generics;

pub fn parse_function<'a>(input: &Span<'a>) -> ParserResult<'a, Statement, TokenParserError> {
    let (input, _) = token_parser!(nodata Fn)(input)?;
    let (input, _) = ws1(&input)?;
    let (input, name) = parse_identifier(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, generics) = parse_generics(&input)?;
    let (input, _) = ws0(&input)?;

    let (input, params) = delimited(
        (token_parser!(nodata LeftParen), ws0).tuple(),
        separated_list0(
            (ws0, token_parser!(nodata Comma), ws0).tuple(),
            (
                parse_identifier,
                opt(
                    (ws0, token_parser!(nodata Colon), ws0, parse_type_expression).tuple()
                ).map(|v| match v {
                    Some((_, _, _, type_)) => type_,
                    None => TypeExpression::Infer,
                })
            ).tuple(),
        ),
        (ws0, token_parser!(nodata RightParen)).tuple(),
    )(&input)?;

    let (input, _) = ws0(&input)?;

    let (input, return_type) = opt(preceded(
        ws_delimited(token_parser!(nodata Arrow)),
        parse_type_expression,
    )).map(|v| match v {
        Some(type_) => type_,
        None => TypeExpression::Infer,
    })(&input)?;

    let (input, _) = ws0(&input)?;
    let (input, code) = parse_block(&input)?;

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
