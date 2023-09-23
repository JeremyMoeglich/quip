use crate::{
    expression::parse_expression,
    identifier::parse_identifier,
    utils::{opt, ws0, ws_delimited},
};
use ast::{Expression, TypeGeneric, TypeGenerics};
use parser_core::*;

fn parse_single_generic<'a>(input: &Span<'a>) -> ParserResult<'a, TypeGeneric> {
    let (input, name) = parse_identifier(&input)?;
    let (input, type_) = opt((ws0, token_parser!(nodata Colon), ws0, parse_expression).tuple())
        .map(|v| match v {
            Some((_, _, _, type_)) => type_,
            None => Expression::Infer,
        })(&input)?;
    Ok((
        input,
        TypeGeneric {
            identifier: name,
            expression: type_,
        },
    ))
}

pub fn parse_generics<'a>(input: &Span<'a>) -> ParserResult<'a, TypeGenerics> {
    match opt(delimited(
        token_parser!(nodata LessThan),
        separated_list0(
            ws_delimited(token_parser!(nodata Comma)),
            parse_single_generic,
        ),
        token_parser!(nodata GreaterThan),
    ))(&input)?
    {
        (input, Some(generics)) => Ok((input, generics)),
        (input, None) => Ok((input, vec![])),
    }
}
