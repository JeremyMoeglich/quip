#![allow(dead_code)]

use fst::Location;
use enumset::EnumSet;
use parser_core::*;

pub use crate::whitespace::*;

// Utils for parsing

#[inline]
pub fn vec_alt<'a, O, F: Fn(Span<'a>) -> ParserResult<'a, O>>(
    parsers: Vec<F>,
) -> impl Fn(Span<'a>) -> ParserResult<'a, O> {
    debug_assert!(parsers.len() > 0);
    move |input| {
        let source_span = input.first_token_span();
        let mut best_error = match input.tokens.get(0) {
            Some(token) => ParserError::UnexpectedToken(Some(token.kind()), EnumSet::empty())
                .locate(source_span),
            None => ParserError::UnexpectedToken(None, EnumSet::empty()).locate(source_span),
        };
        for parser in parsers.iter() {
            let result = parser(input);
            match result {
                Ok((input, value)) => return Ok((input, value)),
                Err(e) => {
                    best_error = best_error.accumulate(e);
                }
            }
        }
        Err(best_error)
    }
}

#[inline]
pub fn vec_tuple<'a, O, F, E>(parsers: Vec<F>) -> impl Fn(Span<'a>) -> ParserResult<Vec<O>>
where
    F: Fn(Span<'a>) -> ParserResult<O>,
{
    move |input| {
        let mut result = Ok((input, Vec::new()));
        for parser in parsers.iter() {
            result = result.and_then(|(input, mut vec)| {
                let (input, value) = parser(input)?;
                vec.push(value);
                Ok((input, vec))
            });
            if result.is_err() {
                break;
            }
        }
        result
    }
}

#[inline]
pub fn opt<'a, O, F>(parser: F) -> impl Fn(Span<'a>) -> SafeParserResult<'a, Option<O>>
where
    F: Fn(Span<'a>) -> ParserResult<'a, O>,
{
    move |input| match parser(input) {
        Ok((input, value)) => (input, Some(value)),
        Err(_) => (input, None),
    }
}

#[inline]
pub fn opt_bool<'a, O, F>(parser: F) -> impl Fn(Span<'a>) -> SafeParserResult<'a, bool>
where
    F: Fn(Span<'a>) -> ParserResult<'a, O>,
{
    move |input| match parser(input) {
        Ok((input, _)) => (input, true),
        Err(_) => (input, false),
    }
}

#[inline]
pub fn locate(text: &str, index: usize) -> Location {
    let mut line = 0;
    let mut column = 0;
    for (i, c) in text.chars().enumerate() {
        if i == index {
            break;
        }
        if c == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
    }
    Location {
        line,
        column,
        index,
    }
}

pub trait ParseString<O> {
    fn parse_string<'a>(&self, input: &'a str) -> ParserOutput<O>;
}

impl<O, F: for<'a> Fn(Span<'a>) -> ParserResult<'a, O>> ParseString<O> for F {
    fn parse_string<'b>(&self, input: &'b str) -> ParserOutput<O> {
        let tokens = tokenize(input);
        let input = create_span(&tokens);
        self(input).to_output()
    }
}

#[inline]
pub fn token_branch<'a, T>(
    values: &'a [T],
    get_kind: impl Fn(&T) -> TokenKind,
) -> impl Fn(Span<'a>) -> ParserResult<'a, &T> {
    let expected_tokens = values.iter().map(|op| get_kind(op)).collect();
    move |input| {
        let (input, (token, source_span)) = input.take_token();
        match token {
            Some(token) => {
                let value = values.iter().find(|op| get_kind(op) == token.kind());
                debug_assert!(token.source_span == source_span);
                match value {
                    Some(value) => Ok((input, value)),
                    None => Err(
                        ParserError::UnexpectedToken(Some(token.kind()), expected_tokens)
                            .locate(token.source_span),
                    ),
                }
            }
            None => Err(ParserError::UnexpectedToken(None, expected_tokens).locate(source_span)),
        }
    }
}
