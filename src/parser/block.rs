use nom::{
    sequence::{delimited, tuple},
    IResult,
    character::complete::char
};

use crate::parser::ast::CodeBlock;

use super::{
    parse_code,
    utils::{ws, Span},
};

pub fn parse_block(input: Span) -> IResult<Span, CodeBlock> {
    delimited(tuple((char('{'), ws)), parse_code, tuple((ws, char('}'))))(input)
}
