#![allow(dead_code)]

use super::ast::{Space, Whitespace};
use crate::{core::{ParserResult, Span, TokenParserError}, ast::Location};
use thiserror::Error;

// Utils for parsing

fn parse_blank<'a>(input: Span<'a>) -> ParserResult<'a, (), TokenParserError> {
    let mut total = 0;
    for c in input.fragment().chars() {
        if c == '\t' {
            total += 4;
        } else {
            total += 1;
        }
    }
    Ok((input, Space::Blank(total)))
}

pub fn ws<'a>(input: Span<'a>) -> ParserResult<Span, Whitespace> {
    let (input, whitespace) = many0(alt((parse_blank, comment)))(input)?;
    Ok((input, whitespace))
}

pub fn ws1(input: Span) -> ParserResult<Span, Whitespace> {
    let (input, whitespace) = many1(alt((comment, parse_blank)))(input)?;
    Ok((input, whitespace))
}

#[derive(Error, Debug)]
pub enum VecAltError {
    #[error("No parser matched")]
    NoParserMatched,
}

pub fn vec_alt<'a, O, F, E: From<VecAltError>>(
    parsers: Vec<F>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E>
where
    F: Fn(&Span<'a>) -> ParserResult<'a, O, E>,
{
    move |input| {
        for parser in parsers.iter() {
            let result = parser(input);
            if result.is_ok() {
                return result;
            }
        }
        Err(VecAltError::NoParserMatched.into())
    }
}

pub fn ws_delimited<'a, O, F, E>(
    mut parser: F,
) -> impl Fn(Span<'a>) -> ParserResult<(Whitespace, O, Whitespace), E>
where
    F: Fn(Span<'a>) -> ParserResult<O, E>,
{
    move |input| {
        let (input, whitespace_before) = ws(input)?;
        let (input, value) = parser(input)?;
        let (input, whitespace_after) = ws(input)?;
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
pub enum AcondError {
    #[error("Parser inactive")]
    ParserInactive,
}

pub fn acond<'a, O, F, E: From<AcondError>>(
    boolean: bool,
    mut parser: F,
) -> impl Fn(Span<'a>) -> ParserResult<O, E>
where
    F: Fn(Span<'a>) -> ParserResult<O, E>,
{
    move |input| match boolean {
        true => parser(input),
        false => Err(AcondError::ParserInactive.into()),
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
    Location { line, column, index }
}
