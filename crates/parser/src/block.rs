use ast::CodeBlock;
use parser_core::*;

use super::{parse_code, utils::ws0};

pub fn parse_block<'a>(input: &Span<'a>) -> ParserResult<'a, CodeBlock> {
    delimited(
        ((parse_LeftBrace, ws0)).tuple(),
        parse_code,
        ((ws0, parse_RightBrace)).tuple(),
    )(input)
}
