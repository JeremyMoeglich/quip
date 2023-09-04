use crate::{
    ast::Expression,
    core::{delimited, separated_list0, ParserResult, Span, Tuple},
    lexer::TokenKind,
    token_parser,
    utils::ws,
};

use super::parse_expression;

pub fn parse_list<'a>(input: Span<'a>) -> ParserResult<'a, Expression, String> {
    let (input, value) = delimited(
        (token_parser!(nodata LeftBracket), ws).tuple(),
        separated_list0(
            (ws, token_parser!(nodata Comma), ws).tuple(),
            parse_expression,
        ),
        (token_parser!(nodata RightBracket), ws).tuple(),
    )(&input)?;
    Ok((input, Expression::List(value)))
}
