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
    let parser = token(TokenKind::LBrace)
        .chain(ws0)
        .chain(many0(parse_statement))
        .chain(token(TokenKind::RBrace))
        .chain(ws0);

    parser.flattened().parse(input)
}
