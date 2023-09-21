use ast::{Statement, TypeExpression};
use parser_core::*;
use crate::{
    expression::parse_expression,
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{opt, ws0, ws1},
};

pub fn parse_declaration<'a>(input: &Span<'a>) -> ParserResult<'a, Statement, TokenParserError> {
    let (input, _) = token_parser!(nodata Let)(input)?;
    let (input, _) = ws1(&input)?;
    
    let (input, mutable) = opt((token_parser!(nodata Mut), ws1).tuple())
        .map(|v| v.is_some())
        (&input)?;
        
    let (input, identifier) = parse_identifier(&input)?;
    
    let (input, type_) = opt(
        (ws0, token_parser!(nodata Colon), ws0, parse_type_expression).tuple()
    )
    .map(|v| match v {
        Some((_, _, _, type_)) => type_,
        None => TypeExpression::Infer,
    })(&input)?;
    
    let (input, _) = ws0(&input)?;
    
    let (input, expression_opt) = opt(
        (token_parser!(nodata Equal), ws0, parse_expression).tuple()
    )
    .map(|v| match v {
        Some((_, _, expression)) => Some(expression),
        None => None,
    })(&input)?;
    
    Ok((input, Statement::Declaration((identifier, type_), mutable, expression_opt)))
}
