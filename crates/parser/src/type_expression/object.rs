use ast::TypeExpression;
use parser_core::*;
use crate::{
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{opt, ws0, ws_delimited},
};

pub fn parse_type_object<'a>(input: &Span<'a>) -> ParserResult<'a, TypeExpression, TokenParserError> {
    let (input, _) = (ws0, token_parser!(nodata LeftBrace), ws0).tuple()(input)?;
    
    let (input, parameters) = separated_list0(
        ws_delimited(token_parser!(nodata Comma)),
        (
            parse_identifier,
            ws_delimited(token_parser!(nodata Colon)),
            parse_type_expression
        ).tuple(),
    )(&input)?;

    let (input, _) = (ws0, token_parser!(nodata RightBrace), ws0).tuple()(&input)?;

    Ok((
        input,
        TypeExpression::Object(
            parameters
                .iter()
                .map(|(name, _, type_)| (name.clone(), type_.clone()))
                .collect::<Vec<_>>()
        )
    ))
}
