use crate::fst::{Parameter, Parameters};

use super::{
    lexer::TokenKind,
    utils::{comma_separated, token, ParseResult, TokenSlice},
};

pub fn parse_parameters(input: TokenSlice) -> ParseResult<Parameters> {
    let (input, values) = comma_separated(token(TokenKind::Ident))(input)?;
    let args = values
        .into_iter()
        .map(|(name, space, second_space)| Parameter::new(name.string(), space, second_space))
        .collect();
    Ok((input, args))
}
