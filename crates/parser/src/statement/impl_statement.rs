use crate::{
    identifier::parse_identifier,
    statement::parse_statement,
    utils::{opt, ws0, ws1},
};
use ast::StatementInner;
use parser_core::*;

use super::generic::parse_generics;

pub fn parse_impl<'a>(input: &Span<'a>) -> ParserResult<'a, StatementInner> {
    let (input, _) = token_parser!(nodata Impl)(&input)?;
    let (input, _) = ws1(&input)?;
    let (input, trait_name) = opt((parse_identifier, ws1, token_parser!(nodata For))
        .tuple()
        .map(|v| v.0.to_string()))(&input)?;
    let (input, identifier) = parse_identifier(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, generics) = parse_generics(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, _) = token_parser!(nodata LeftBrace)(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, statements) = many0(parse_statement)(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, _) = token_parser!(nodata RightBrace)(&input)?;

    Ok((
        input,
        StatementInner::Impl {
            target: identifier,
            trait_name,
            generics,
            statements,
        },
    ))
}
