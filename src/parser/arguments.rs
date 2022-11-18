
use crate::fst::{Arguments, Argument};

use super::{
    lexer::TokenKind,
    utils::{comma_separated, token, ParseResult, TokenSlice},
};

pub fn parse_arguments(input: TokenSlice) -> ParseResult<Arguments> {
    let (input, values) = comma_separated(token(TokenKind::Ident))(input)?;
    let args = values
        .into_iter()
        .map(|(token, space, second_space)| Argument::new(token.string(), space, second_space))
        .collect();
    Ok((input, args))
}
