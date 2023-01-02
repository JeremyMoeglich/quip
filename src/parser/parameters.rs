use super::{
    common::{comma_separated, token},
    core::Parser,
    lexer::LocatedToken,
};
use crate::fst::{Parameter, Parameters};

use super::{
    core::{ParseResult, TokenSlice},
    lexer::TokenKind,
};

fn ident_token_parser(input: TokenSlice) -> ParseResult<LocatedToken> {
    token(TokenKind::Ident).parse(input)
}

pub fn parse_parameters(input: TokenSlice) -> ParseResult<Parameters> {
    comma_separated(&ident_token_parser).map_result(&|args| {
        args.into_iter()
            .map(|(ident, space1, space2)| Parameter::new(ident.string(), space1, space2))
            .collect()
    })(input)
}
