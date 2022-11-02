use nom::{
    bytes::complete::tag,
    sequence::{delimited, tuple},
    IResult,
};

use crate::ast::CodeBlock;

use super::{
    parse_code,
    utils::{ws, Span},
};

pub fn parse_block(input: Span) -> IResult<Span, CodeBlock> {
    delimited(tuple((tag("{"), ws)), parse_code, tuple((ws, tag("}"))))(input)
}
