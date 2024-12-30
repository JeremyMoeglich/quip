use parser_core::*;

use crate::utils::{opt, ws0};

pub fn require_semicolon<'a>(input: Span<'a>) -> ParserResult<'a, ()> {
    let (input, _) = ws0(input);
    let (input, _) = parse_semicolon(input)?;
    Ok((input, ()))
}

pub fn opt_semicolon<'a>(input: Span<'a>) -> SafeParserResult<'a, bool> {
    let (input, semi) = opt((ws0, parse_semicolon).tuple())(input);
    (input, semi.is_some())
}
