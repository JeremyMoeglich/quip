use ast::TypeExpression;
use parser_core::*;
use crate::utils::{ws0};

pub fn parse_type_array<'a, F>(
    mut start_parser: F,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, TypeExpression, TokenParserError>
where
    F: Fn(&Span<'a>) -> ParserResult<'a, TypeExpression, TokenParserError>,
{
    move |input: &Span<'a>| {
        let (mut input, mut expression) = start_parser(input)?;
        while let Ok((new_input, _)) = (ws0, token_parser!(nodata LeftBracket), ws0, token_parser!(nodata RightBracket), ws0).tuple()(&input) {
            input = new_input;
            expression = TypeExpression::Array(Box::new(expression));
        }
        Ok((input, expression))
    }
}
