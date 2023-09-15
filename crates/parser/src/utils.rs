#![allow(dead_code)]

use ast::Whitespace;
use parser_core::*;
use lexer::{Token, TokenKind};
use thiserror::Error;

// Utils for parsing

fn parse_blank<'a>(input: &Span<'a>) -> ParserResult<'a, (), TokenParserError> {
    let (input, blank) = take_while(|t| t.kind() == TokenKind::Space)(input);
    Ok((input, ()))
}

pub fn ws1(input: Span) -> ParserResult<Span, TakeParserError> {
    let (input, whitespace) = many1((token_parser!(data LineComment), parse_blank).alt())(&input)?;
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
    Location {
        line,
        column,
        index,
    }
}
