

use crate::ast::CodeBlock;

use super::{
    parse_code,
    utils::{ws0, Span},
};

pub fn parse_block(input: Span) -> IResult<Span, CodeBlock> {
    delimited(tuple((char('{'), ws0)), parse_code, tuple((ws0, char('}'))))(input)
}
