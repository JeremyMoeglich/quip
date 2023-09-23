use parser_core::*;

pub fn parse_identifier<'a>(input: &Span<'a>) -> ParserResult<'a, String> {
    token_parser!(data Ident).map(|v| v.to_string())(input)
}
