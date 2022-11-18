use nom::sequence::tuple;

use crate::fst::Fst;

use self::{
    code_block::parse_code_block,
    lexer::lex,
    utils::{force_eof, ws0, ParseResult, TokenSlice},
};

mod arguments;
mod code_block;
mod expression;
mod lexer;
mod statement;
mod utils;

pub fn parse_fst<'a>(tokens: TokenSlice<'a>) -> ParseResult<'a, Fst> {
    let (input, (beginning_space, index_block)) =
        force_eof(tuple((ws0, parse_code_block)))(tokens)?;
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
