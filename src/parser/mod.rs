use crate::fst::Fst;

use self::{
    lexer::lex,
    statement::parse_statement,
    core::{force_eof, ws0, ParseResult, TokenSlice},
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
        force_eof(tuple((ws0, many0(parse_statement))))(tokens)?;
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
