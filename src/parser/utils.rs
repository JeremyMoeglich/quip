use nom::{
    combinator::map,
    multi::{many0, many1},
    IResult,
};

use crate::fst::{Ident, Space, SpacePart};

use super::lexer::{LocatedToken, Token, TokenKind};

pub type TokenSlice<'a> = &'a [LocatedToken<'a>];
pub type ParseResult<'a, T> = IResult<TokenSlice<'a>, T>;

pub fn incomplete<T>() -> ParseResult<'static, T> {
    Err(nom::Err::Incomplete(nom::Needed::Unknown))
}

pub fn not_it<'a, T>(input: TokenSlice<'a>) -> ParseResult<'a, T> {
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Not,
    )))
}

pub fn eof<T>(input: TokenSlice) -> ParseResult<T> {
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Eof,
    )))
}

pub fn slice_next_stream(input: TokenSlice) -> ParseResult<LocatedToken> {
    if let Some((head, tail)) = input.split_first() {
        Ok((tail, head.clone()))
    } else {
        incomplete()
    }
}

#[allow(dead_code)]
pub fn slice_next(input: TokenSlice) -> ParseResult<LocatedToken> {
    complete(slice_next_stream)(input)
}

pub fn token_stream<'a>(
    kind: TokenKind,
) -> impl Fn(TokenSlice<'a>) -> ParseResult<'a, LocatedToken> {
    move |input: TokenSlice| {
        let (input, token) = slice_next_stream(input)?;
        if token.kind() == kind {
            Ok((input, token))
        } else {
            not_it(input)
        }
    }
}

pub fn token<'a>(kind: TokenKind) -> impl FnMut(TokenSlice<'a>) -> ParseResult<'a, LocatedToken> {
    complete(token_stream(kind))
}

pub fn tokens_stream<'a>(
    kinds: &'a [TokenKind],
) -> impl FnMut(TokenSlice<'a>) -> ParseResult<'a, LocatedToken> {
    move |input: TokenSlice| {
        let (input, token) = slice_next_stream(input)?;
        if kinds.contains(&token.kind()) {
            Ok((input, token))
        } else {
            incomplete()
        }
    }
}

pub fn tokens<'a>(
    kinds: &'a [TokenKind],
) -> impl FnMut(TokenSlice<'a>) -> ParseResult<'a, LocatedToken> {
    complete(tokens_stream(kinds))
}

pub fn parse_ident(input: TokenSlice) -> ParseResult<Ident> {
    let (input, token) = token(TokenKind::Ident)(input)?;
    Ok((input, token.string()))
}

pub fn opt_token(kind: TokenKind) -> impl Fn(TokenSlice) -> ParseResult<Option<LocatedToken>> {
    move |input: TokenSlice| {
        if let Ok((input, token)) = token(kind)(input) {
            if token.kind() == kind {
                Ok((input, Some(token)))
            } else {
                Ok((input, None))
            }
        } else {
            Ok((input, None))
        }
    }
}

fn get_single_line_comment(input: &str) -> Result<String, String> {
    if !input.starts_with("//") {
        return Err("Expected single line comment".to_string());
    }
    // slice off the "//"
    let input = &input[2..];
    // slice off the newline if it exists
    let input = &input[..input.find('\n').unwrap_or(input.len())];
    Ok(input.to_string())
}

fn get_multi_line_comment(input: &str) -> Result<String, String> {
    if !input.starts_with("/*") {
        return Err("Expected multi line comment".to_string());
    }
    // slice off the "/*"
    let input = &input[2..];
    // slice off the "*/" if it exists
    let input = &input[..input.find("*/").unwrap_or(input.len())];
    Ok(input.to_string())
}

fn parse_space_part(input: TokenSlice) -> ParseResult<SpacePart> {
    map(
        tokens(&[
            TokenKind::Whitespace,
            TokenKind::SingleLineComment,
            TokenKind::MultiLineComment,
        ]),
        |loc_token| match loc_token.token {
            Token::Whitespace(s) => SpacePart::Whitespace(s.to_string()),
            Token::SingleLineComment(s) => {
                SpacePart::SingleLineComment(get_single_line_comment(s).unwrap())
            }
            Token::MultiLineComment(s) => {
                SpacePart::MultiLineComment(get_multi_line_comment(s).unwrap())
            }
            _ => unreachable!(),
        },
    )(input)
}

pub fn ws0<'a>(input: TokenSlice<'a>) -> ParseResult<'a, Space> {
    let (input, space) = many0(parse_space_part)(input)?;
    Ok((input, space))
}

pub fn ws1<'a>(input: TokenSlice<'a>) -> ParseResult<'a, Space> {
    let (input, space) = many1(parse_space_part)(input)?;
    Ok((input, space))
}

pub fn force_eof<'a, T>(
    mut parser: impl FnMut(TokenSlice<'a>) -> ParseResult<'a, T>,
) -> impl FnMut(TokenSlice<'a>) -> ParseResult<'a, T> {
    move |input| {
        let (input, result) = parser(input)?;
        if input.is_empty() {
            Ok((input, result))
        } else {
            eof(input)
        }
    }
}

pub fn complete<'a, T>(
    mut parser: impl FnMut(TokenSlice<'a>) -> ParseResult<'a, T>,
) -> impl FnMut(TokenSlice<'a>) -> ParseResult<'a, T> {
    move |input| {
        let result = parser(input);
        if let Ok(result) = result {
            return Ok(result);
        } else {
            not_it(input)
        }
    }
}

pub fn comma_separated<'a, T>(
    mut parser: impl FnMut(TokenSlice<'a>) -> ParseResult<'a, T>,
) -> impl FnMut(TokenSlice<'a>) -> ParseResult<'a, Vec<(T, Space, Option<Space>)>> {
    move |input: TokenSlice<'a>| {
        let mut last_arg = false;
        let (input, args) = many0(|input| {
            if last_arg {
                return not_it(input);
            };
            let (input, value) = parser(input)?;
            let (input, space) = ws0(input)?;
            let (input, second_space) = {
                let (input, comma) = opt_token(TokenKind::Comma)(input)?;
                if let Some(_) = comma {
                    let (input, space) = ws0(input)?;
                    (input, Some(space))
                } else {
                    last_arg = true;
                    (input, None)
                }
            };
            Ok((input, (value, space, second_space)))
        })(input)?;
        Ok((input, args))
    }
}
