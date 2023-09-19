#![allow(dead_code)]

use ast::{Comment, Location, Whitespace};
use lexer::{Token, TokenKind};
use parser_core::*;
use thiserror::Error;

// Utils for parsing

pub fn ws1<'a>(input: &Span<'a>) -> ParserResult<'a, Vec<Comment>, TakeParserError> {
    let (input, whitespace) = many1(
        (
            token_parser!(data LineComment).map(|v| Some(Comment::Line(v.to_string()))),
            token_parser!(data Space).map(|v| None),
            token_parser!(data BlockComment).map(|v| Some(Comment::Block(v.to_string()))),
        )
            .alt(),
    )(input)?;
    Ok((input, whitespace.iter().filter_map(|v| *v).collect()))
}

pub fn ws0<'a>(input: &Span<'a>) -> ParserResult<'a, Vec<Comment>, TakeParserError> {
    let (input, whitespace) = many0(
        (
            token_parser!(data LineComment).map(|v| Some(Comment::Line(v.to_string()))),
            token_parser!(data Space).map(|v| None),
            token_parser!(data BlockComment).map(|v| Some(Comment::Block(v.to_string()))),
        )
            .alt(),
    )(input)?;
    Ok((input, whitespace.iter().filter_map(|v| *v).collect()))
}

#[derive(Error, Debug)]
pub enum VecAltError {
    #[error("No parser matched")]
    NoParserMatched,
}

pub fn vec_alt<'a, O, F, E: From<VecAltError>, E2>(
    parsers: Vec<F>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E>
where
    F: Fn(&Span<'a>) -> ParserResult<'a, O, E2>,
{
    move |input| {
        for parser in parsers.iter() {
            let result = parser(input);
            match result {
                Ok((input, value)) => return Ok((input, value)),
                Err(_) => continue,
            }
        }
        Err(VecAltError::NoParserMatched.into())
    }
}

pub fn ws_delimited<'a, O, F, E: From<TakeParserError>>(
    parser: F,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, (Whitespace, O, Whitespace), E>
where
    F: Fn(&Span<'a>) -> ParserResult<'a, O, E>,
{
    move |input| {
        let (input, whitespace_before) = ws0(input)?;
        let (input, value) = parser(&input)?;
        let (input, whitespace_after) = ws0(&input)?;
        Ok((input, (whitespace_before, value, whitespace_after)))
    }
}

pub fn vec_tuple<'a, O, F, E>(parsers: Vec<F>) -> impl Fn(Span<'a>) -> ParserResult<Vec<O>, E>
where
    F: Fn(Span<'a>) -> ParserResult<O, E>,
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

#[derive(Error, Debug)]
pub enum AcondError<E> {
    #[error("Parser inactive")]
    ParserInactive,
    #[error("Parser error: {0}")]
    ParserError(E),
}

pub fn acond<'a, O, F, E>(boolean: bool, parser: F) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, AcondError<E>>
where
    F: Fn(&Span<'a>) -> ParserResult<'a, O, E>,
{
    move |input| match boolean {
        true => parser(input).map_err(|e| AcondError::ParserError(e)),
        false => Err(AcondError::ParserInactive),
    }
}

pub fn opt<'a, O, F, E: From<TakeParserError>>(
    parser: F,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Option<O>, E>
where
    F: Fn(&Span<'a>) -> ParserResult<'a, O, E>,
{
    move |input| match parser(input) {
        Ok((input, value)) => Ok((input, Some(value))),
        Err(_) => Ok((input.clone(), None)),
    }
}

pub fn separated_pair<'a, O1, O2, O3, F1, F2, F3, E: From<TakeParserError>>(
    parser1: F1,
    parser2: F2,
    parser3: F3,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O1, O3), E>
where
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, E>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, E>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, E>,
{
    move |input| {
        let (input, value1) = parser1(input)?;
        let (input, _) = parser2(&input)?;
        let (input, value2) = parser3(&input)?;
        Ok((input, (value1, value2)))
    }
}

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

pub fn static_span(text: &'static str) -> Span<'static> {
    let tokens = tokenize(text);
    create_span(&tokens)
}

trait ParseString<'a, O, E> {
    fn parse_string(&self, input: &'a str) -> ParserResult<'a, O, E>;
}

impl<'a, O, F, E> ParseString<'a, O, E> for F
where
    F: Fn(&Span<'a>) -> ParserResult<'a, O, E>,
{
    fn parse_string(&self, input: &'a str) -> ParserResult<'a, O, E> {
        let tokens = tokenize(input);
        let span = create_span(&tokens);
        self(&span)
    }
}
