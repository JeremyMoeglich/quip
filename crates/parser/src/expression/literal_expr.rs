use lexer::{Token, TokenKind};
use parser_core::*;

use fst::{Expression, Literal};

pub fn parse_literal_expr<'a>(input: Span<'a>) -> ParserResult<'a, Expression> {
    let (input, (token, source_span)) = input.take_token();
    let kinds = TokenKind::RawString | TokenKind::Number | TokenKind::String | TokenKind::Boolean;
    let literal = match token.delocate() {
        Some(Token::RawString(string)) => Ok((input, Literal::String(string.to_string()))),
        Some(Token::Number(number)) => Ok((input, Literal::Number(number.to_string()))),
        Some(Token::String(string)) => Ok((input, Literal::String(string.to_string()))),
        Some(Token::Boolean(boolean)) => Ok((input, Literal::Boolean(boolean))),
        _ => Err(token.as_parser_error(kinds, source_span)),
    };
    match literal {
        Ok((input, literal)) => Ok((input, Expression::Literal { value: literal })),
        Err(error) => Err(error),
    }
}
