use lexer::Token;
use parser_core::*;
use thiserror::Error;

use ast::Literal;

#[derive(Debug, Error)]
pub enum SubParseLiteralError {
    #[error("Expected a string literal")]
    NotALiteral,
}

combine_errors!(pub ParseLiteralError, SubParseLiteralError, TakeParserError);

pub fn parse_literal<'a>(input: &Span<'a>) -> ParserResult<'a, Literal, ParseLiteralError> {
    let (input, token) = input.take_token()?;
    match token.token {
        Token::RawString(string) => Ok((input, Literal::String(string.to_string()))),
        Token::Number(number) => Ok((input, Literal::Number(number))),
        Token::DoubleQuoteString(string) => Ok((input, Literal::String(string.to_string()))),
        Token::SingleQuoteString(string) => Ok((input, Literal::String(string.to_string()))),
        _ => Err(SubParseLiteralError::NotALiteral.into()),
    }
}
