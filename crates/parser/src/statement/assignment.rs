use ast::Statement;
use parser_core::*;

use crate::{expression::parse_expression, utils::ws0};

pub fn parse_assignment<'a>(input: &Span<'a>) -> ParserResult<'a, Statement, TokenParserError> {
    let (input, to_change) = parse_expression(input)?;
    let (input, _) = ws0(&input)?;
    let (input, _) = token_parser!(nodata Equal)(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, expression) = parse_expression(&input)?;
    Ok((input, Statement::Assignment(to_change, expression)))
}
