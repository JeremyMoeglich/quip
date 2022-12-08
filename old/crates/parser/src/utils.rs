#![allow(dead_code)]
extern crate nom;
extern crate nom_locate;

use nom::{
    branch::alt,
    bytes::{complete::is_not, complete::tag},
    character::complete::{not_line_ending},
    multi::{many0, many1},
    sequence::delimited,
    IResult,
};

use nom_locate::LocatedSpan;

use super::fst::{Space, Whitespace};

// Utils for parsing

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn new_span(input: &str) -> Span {
    Span::new_extra(input, ())
}

fn single_line_comment(input: Span) -> IResult<Span, Space> {
    let (input, _) = tag("//")(input)?;
    let (input, comment) = not_line_ending(input)?;
    Ok((input, Space::LineComment(comment.fragment().to_string())))
}

fn multi_line_comment(input: Span) -> IResult<Span, Space> {
    let comment_start = "/*";
    let comment_end = "*/";
    let (input, comment) =
        delimited(tag(comment_start), is_not(comment_end), tag(comment_end))(input)?;
    Ok((input, Space::BlockComment(comment.fragment().to_string())))
}

fn comment(input: Span) -> IResult<Span, Space> {
    alt((single_line_comment, multi_line_comment))(input)
}

fn parse_blank(input: Span) -> IResult<Span, Space> {
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

pub fn ws(input: Span) -> IResult<Span, Whitespace> {
    let (input, whitespace) = many0(alt((parse_blank, comment)))(input)?;
    Ok((input, whitespace))
}

pub fn ws1(input: Span) -> IResult<Span, Whitespace> {
    let (input, whitespace) = many1(alt((comment, parse_blank)))(input)?;
    Ok((input, whitespace))
}

pub fn vec_alt<'a, O, F>(mut parsers: Vec<F>) -> impl Fn(Span<'a>) -> IResult<Span<'a>, O>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O>,
{
    move |input| {
        let mut result = Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Alt,
        )));
        for parser in parsers.iter_mut() {
            result = parser(input);
            if result.is_ok() {
                break;
            }
        }
        result
    }
}

pub fn ws_delimited<'a, O, F>(
    mut parser: F,
) -> impl Fn(Span<'a>) -> IResult<Span<'a>, (Whitespace, O, Whitespace)>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O>,
{
    move |input| {
        let (input, whitespace_before) = ws(input)?;
        let (input, value) = parser(input)?;
        let (input, whitespace_after) = ws(input)?;
        Ok((input, (whitespace_before, value, whitespace_after)))
    }
}

pub fn vec_tuple<'a, O, F>(mut parsers: Vec<F>) -> impl Fn(Span<'a>) -> IResult<Span<'a>, Vec<O>>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O>,
{
    move |input| {
        let mut result = Ok((input, Vec::new()));
        for parser in parsers.iter_mut() {
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

pub fn acond<'a, O, F>(boolean: bool, mut parser: F) -> impl Fn(Span<'a>) -> IResult<Span<'a>, O>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O>,
{
    move |input| match boolean {
        true => parser(input),
        false => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Alt,
        ))),
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub column: usize,
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
    Location { line, column }
}
