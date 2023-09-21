use ast::{Statement, TypeExpression};
use parser_core::*;
use crate::{
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{opt, ws0, ws1},
};

use super::generic::parse_generics;

pub fn parse_struct<'a>(input: &Span<'a>) -> ParserResult<'a, Statement, TokenParserError> {
    let (input, _) = token_parser!(nodata Struct)(input)?;
    let (input, _) = ws1(&input)?;
    let (input, name) = parse_identifier(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, generics) = parse_generics(&input)?;
    let (input, _) = ws0(&input)?;

    let (input, fields) = delimited(
        (token_parser!(nodata LeftBrace), ws0).tuple(),
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
        (ws0, opt(token_parser!(nodata Comma)), ws0, token_parser!(nodata RightBrace)).tuple(),
    )(&input)?;

    Ok((input, Statement::Struct(name, generics, fields)))
}
