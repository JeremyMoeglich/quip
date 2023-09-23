use ast::CodeBlock;
use parser_core::*;

use super::{parse_code, utils::ws0};

pub fn parse_block<'a>(input: &Span<'a>) -> ParserResult<'a, CodeBlock> {
    delimited(
        ((token_parser!(nodata LeftBrace), ws0)).tuple(),
        parse_code,
        ((ws0, token_parser!(nodata RightBrace))).tuple(),
    )(input)
}
