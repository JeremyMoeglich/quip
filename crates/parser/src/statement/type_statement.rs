use ast::Statement;
use parser_core::*;
use crate::{
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{opt, ws0, ws1},
};

use super::generic::parse_generics;

pub fn parse_type_statement<'a>(input: &Span<'a>) -> ParserResult<'a, Statement, TokenParserError> {
    let (input, _) = token_parser!(nodata Type)(input)?;
    let (input, _) = ws1(&input)?;
    let (input, name) = parse_identifier(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, generics) = parse_generics(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, _) = token_parser!(nodata Equals)(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, type_) = parse_type_expression(&input)?;

    Ok((input, Statement::TypeAlias(name, generics, type_)))
}
