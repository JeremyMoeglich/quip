use crate::fst::Fst;
mod common;

use self::{
    lexer::lex,
    statement::parse_statement,
    core::{ParseResult, TokenSlice}, common::{ws0, many0, force_eof},
};

mod arguments;
mod code_block;
mod expression;
mod lexer;
mod parameters;
mod statement;
mod core;

pub fn parse_fst<'a>(tokens: TokenSlice<'a>) -> ParseResult<'a, Fst> {
    let (input, (beginning_space, index_block)) =
        force_eof(ws0.chain(many0(parse_statement)).flatten())(tokens)?;
    Ok((input, Fst::new(beginning_space, index_block)))
}

pub fn parse(code: &str) -> Result<Fst, String> {
    let tokens = lex(code);
    let result = parse_fst(&tokens);
    match result {
        Ok((_, fst)) => Ok(fst),
        Err(e) => Err(e.to_string()),
    }
}
