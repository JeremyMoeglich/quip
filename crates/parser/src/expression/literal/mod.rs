use lexer::{Token, TokenKind};
use parser_core::*;

use ast::Literal;

pub fn parse_literal<'a>(input: &Span<'a>) -> ParserResult<'a, Literal> {
    let start = input.start;
    let (input, token) = input.take_token()?;
    match token.token {
        Token::RawString(string) => Ok((input, Literal::String(string.to_string()))),
        Token::Number(number) => Ok((input, Literal::Number(number))),
        Token::DoubleQuoteString(string) => Ok((input, Literal::String(string.to_string()))),
        Token::SingleQuoteString(string) => Ok((input, Literal::String(string.to_string()))),
        Token::Boolean(boolean) => Ok((input, Literal::Boolean(boolean))),
        _ => Err(ParserError::UnexpectedToken(
            token.token.kind(),
            vec![
                TokenKind::RawString,
                TokenKind::Number,
                TokenKind::DoubleQuoteString,
                TokenKind::SingleQuoteString,
                TokenKind::Boolean,
            ],
        )
        .locate(start)),
    }
}
