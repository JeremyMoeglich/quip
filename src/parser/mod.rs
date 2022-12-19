use crate::fst::Fst;
mod common;

use self::{
    common::{force_eof, many0, ws0},
    core::{ParseResult, Parser, TokenSlice},
    lexer::lex,
    statement::parse_statement,
};

mod arguments;
mod code_block;
mod core;
mod expression;
mod lexer;
mod parameters;
mod statement;

pub fn parse_fst<'a>(tokens: TokenSlice<'a>) -> ParseResult<'a, Fst> {
    force_eof(ws0.chain(many0(parse_statement)).flattened()).map_result(&|(
        beginning_space,
        statements,
    )| {
        Fst::new(beginning_space, statements)
    })(tokens)
}

pub fn parse(code: &str) -> Result<Fst, String> {
    let tokens = lex(code);
    let result = parse_fst(&tokens);
    match result {
        Ok((_, fst)) => Ok(fst),
        Err(e) => Err(format!("{:?}", e)),
    }
}
