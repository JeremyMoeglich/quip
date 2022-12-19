use super::{
    common::{comma_separated, token}, core::Parser,
};
use crate::fst::{Parameter, Parameters};

use super::{
    core::{ParseResult, TokenSlice},
    lexer::TokenKind,
};

pub fn parse_parameters(input: TokenSlice) -> ParseResult<Parameters> {
    comma_separated(token(TokenKind::Ident)).map_result(|args| {
        args.into_iter()
            .map(|(ident, space1, space2)| Parameter::new(ident.string(), space1, space2))
            .collect()
    })(input)
}
