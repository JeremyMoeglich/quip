use nom::multi::many0;

use crate::{
    fst::CodeBlock,
    parser::utils::{ParseResult, TokenSlice},
};

use super::statement::parse_statement;

pub fn parse_code_block<'a>(input: TokenSlice<'a>) -> ParseResult<CodeBlock> {
    many0(parse_statement)(input)
}
