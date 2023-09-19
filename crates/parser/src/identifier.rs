use parser_core::*;



pub fn parse_identifier<'a>(input: &Span<'a>) -> ParserResult<'a, String, TakeParserError> {
    token_parser!(data Ident)(input)
}
