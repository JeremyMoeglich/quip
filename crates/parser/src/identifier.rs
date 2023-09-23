use parser_core::*;

pub fn parse_identifier<'a>(input: &Span<'a>) -> ParserResult<'a, String> {
    parse_Ident.map(|v| v.to_string())(input)
}
