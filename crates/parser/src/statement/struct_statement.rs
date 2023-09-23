use crate::{
    expression::parse_expression,
    identifier::parse_identifier,
    utils::{opt, ws0, ws1},
};
use ast::{Expression, StatementInner, TypedIdentifier};
use parser_core::*;

use super::generic::parse_generics;

pub fn parse_struct_block<'a>(input: &Span<'a>) -> ParserResult<'a, Vec<TypedIdentifier>> {
    delimited(
        (token_parser!(nodata LeftBrace), ws0).tuple(),
        separated_list0(
            (ws0, token_parser!(nodata Comma), ws0).tuple(),
            (
                parse_identifier,
                opt((ws0, token_parser!(nodata Colon), ws0, parse_expression).tuple()).map(|v| {
                    match v {
                        Some((_, _, _, type_)) => type_,
                        None => Expression::Infer,
                    }
                }),
            )
                .tuple()
                .map(|(ident, type_)| TypedIdentifier {
                    identifier: ident.to_string(),
                    expression: type_,
                }),
        ),
        (
            ws0,
            opt(token_parser!(nodata Comma)),
            ws0,
            token_parser!(nodata RightBrace),
        )
            .tuple(),
    )(input)
}
pub fn parse_struct<'a>(input: &Span<'a>) -> ParserResult<'a, StatementInner> {
    let (input, _) = token_parser!(nodata Struct)(input)?;
    let (input, _) = ws1(&input)?;
    let (input, name) = parse_identifier(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, generics) = parse_generics(&input)?;
    let (input, _) = ws0(&input)?;

    let (input, fields) = parse_struct_block(&input)?;

    Ok((
        input,
        StatementInner::Struct {
            name: name.to_string(),
            generics,
            fields,
        },
    ))
}
