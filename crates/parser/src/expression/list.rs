use ast::Expression;
use parser_core::*;

use crate::utils::ws0;

use super::parse_expression;

pub fn parse_list<'a>(input: &Span<'a>) -> ParserResult<'a, Expression, TakeParserError> {
    let (input, value) = delimited(
        (token_parser!(nodata LeftBracket), MapParser::map(ws0, |_| ())).tuple(),
        separated_list0(
            (ws0, token_parser!(nodata Comma), ws0).tuple(),
            parse_expression,
        ),
        (token_parser!(nodata RightBracket), ws0).tuple(),
    )(&input)?;
    Ok((input, Expression::List(value)))
}
