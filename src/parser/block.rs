use nom::{
    character::complete::char,
    sequence::{delimited, tuple},
    IResult,
};

use crate::parser::fst::CodeBlock;

use super::{
    parse_code,
    utils::{ws, Span},
};

pub fn parse_block(input: Span) -> IResult<Span, CodeBlock> {
    delimited(tuple((char('{'), ws)), parse_code, tuple((ws, char('}'))))(input)
}
