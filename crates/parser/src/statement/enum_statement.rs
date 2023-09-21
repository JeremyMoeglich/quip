use ast::Statement;
use parser_core::*;

use crate::{
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{opt, ws0, ws1, ws_delimited},
};

use super::generic::parse_generics;

pub fn parse_enum<'a>(input: &Span<'a>) -> ParserResult<'a, Statement, TokenParserError> {
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
                opt(delimited(
                    ws_delimited(token_parser!(nodata LeftParen)),
                    separated_list0(
                        ws_delimited(token_parser!(nodata Comma)),
                        parse_type_expression,
                    ),
                    ws_delimited(token_parser!(nodata RightParen)),
                ))
                .map(|v| match v {
                    Some(type_) => type_,
                    None => vec![],
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
    )(input)?;
    Ok((input, Statement::Enum(name, generics, options)))
}
