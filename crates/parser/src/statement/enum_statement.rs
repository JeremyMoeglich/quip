use ast::{EnumValue, StatementInner};
use parser_core::*;

use crate::{
    expression::parse_expression,
    identifier::parse_identifier,
    utils::{opt, ws0, ws1, ws_delimited},
};

use super::{generic::parse_generics, struct_statement::parse_struct_block};

pub fn parse_enum<'a>(input: &Span<'a>) -> ParserResult<'a, StatementInner> {
    let (input, _) = token_parser!(nodata Enum)(input)?;
    let (input, _) = ws1(&input)?;
    let (input, name) = parse_identifier(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, generics) = parse_generics(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, options) = delimited(
        (token_parser!(nodata LeftBrace), ws0).tuple(),
        separated_list0(
            (ws0, token_parser!(nodata Comma), ws0).tuple(),
            (
                parse_identifier,
                opt((
                    delimited(
                        ws_delimited(token_parser!(nodata LeftParen)),
                        separated_list0(
                            ws_delimited(token_parser!(nodata Comma)),
                            parse_expression,
                        ),
                        ws_delimited(token_parser!(nodata RightParen)),
                    )
                    .map(|v| EnumValue::Tuple(v)),
                    parse_struct_block.map(|v| EnumValue::Struct(v)),
                )
                    .alt())
                .map(|v| match v {
                    Some(type_) => type_,
                    None => EnumValue::Tuple(vec![]),
                }),
            )
                .tuple(),
        ),
        (
            ws0,
            opt(token_parser!(nodata Comma)),
            ws0,
            token_parser!(nodata RightBrace),
        )
            .tuple(),
    )(&input)?;
    Ok((
        input,
        StatementInner::Enum {
            name,
            type_generics: generics,
            options,
        },
    ))
}
