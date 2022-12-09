use crate::{
    fst::CodeBlock,
    parser::{
        core::{ParseResult, TokenSlice},
        lexer::TokenKind,
    },
};

use super::{
    common::{many0, token, ws0},
    core::Parser,
    statement::parse_statement,
};

pub fn parse_code_block<'a>(input: TokenSlice<'a>) -> ParseResult<CodeBlock> {
    token(TokenKind::LBrace)
        .chain(ws0)
        .chain(many0(parse_statement))
        .chain(token(TokenKind::RBrace))
        .chain(ws0)
        .flattened()
        .map_result(|(_, space1, statements, _, space2)| CodeBlock::new(space1, statements, space2))
        .parse(input)
}
