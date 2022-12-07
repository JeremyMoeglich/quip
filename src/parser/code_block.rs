use crate::{
    fst::CodeBlock,
    parser::{
        lexer::TokenKind,
        core::{ParseResult, TokenSlice},
    },
};

use super::{
    statement::parse_statement,
    core::{token, ws0},
};

pub fn parse_code_block<'a>(input: TokenSlice<'a>) -> ParseResult<CodeBlock> {
    let (input, _) = token(TokenKind::LBrace)(input)?;
    let (input, space_lbrace_stat1) = ws0(input)?;
    let (input, statements) = many0(parse_statement)(input)?;
    let (input, _) = token(TokenKind::RBrace)(input)?;
    let (input, right_space) = ws0(input)?;
    Ok((
        input,
        CodeBlock::new(space_lbrace_stat1, statements, right_space),
    ))
}
