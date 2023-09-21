use ast::Statement;
use parser_core::*;
use crate::{
    identifier::parse_identifier,
    utils::{ws0, ws1, opt},
    statement::parse_statement,
};

pub fn parse_impl<'a>(input: &Span<'a>) -> ParserResult<'a, Statement, TokenParserError> {
    let (input, _) = token_parser!(nodata Impl)(&input)?;
    let (input, _) = ws1(&input)?;
    let (input, trait_name) = opt(
        (parse_identifier, ws1, token_parser!(nodata For)).map(|v| {
            match v {
                Some((trait_name, _, _)) => Some(trait_name),
                None => None,
            }
        })
    )(&input)?;
    let (input, identifier) = parse_identifier(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, _) = token_parser!(nodata OpenBrace)(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, statements) = many0(parse_statement)(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, _) = token_parser!(nodata CloseBrace)(&input)?;

    Ok((input, Statement::Impl(identifier, trait_name, statements)))
}
