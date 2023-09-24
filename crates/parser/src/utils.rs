#![allow(dead_code)]

use ast::{Comment, Location};
use parser_core::*;

// Utils for parsing

pub fn ws1<'a>(input: &Span<'a>) -> ParserResult<'a, Vec<Comment>> {
    let (input, whitespace) = many1(
        (
            parse_LineComment.map(|v| Some(Comment::Line(v.to_string()))),
            parse_Space.map(|_| None),
            parse_BlockComment.map(|v| Some(Comment::Block(v.to_string()))),
        )
            .alt(),
    )(input)?;
    Ok((input, whitespace.iter().filter_map(|v| v.clone()).collect()))
}

pub fn ws0<'a>(input: &Span<'a>) -> ParserResult<'a, Vec<Comment>> {
    let (input, whitespace) = many0(
        (
            parse_LineComment.map(|v| Some(Comment::Line(v.to_string()))),
            parse_Space.map(|_| None),
            parse_BlockComment.map(|v| Some(Comment::Block(v.to_string()))),
        )
            .alt(),
    )(input)?;
    Ok((input, whitespace.iter().filter_map(|v| v.clone()).collect()))
}

pub fn vec_alt<'a, O, F: Fn(&Span<'a>) -> ParserResult<'a, O>>(
    parsers: Vec<F>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, O> {
    move |input| {
        let mut best_error = ParserError::InactiveParser.locate(input.start);
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

pub fn ws_delimited<'a, O, F>(parser: F) -> impl Fn(&Span<'a>) -> ParserResult<'a, O>
where
    F: Fn(&Span<'a>) -> ParserResult<'a, O>,
{
    move |input| {
        let (input, _) = ws0(input)?;
        let (input, value) = parser(&input)?;
        let (input, _) = ws0(&input)?;
        Ok((input, value))
    }
}

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

pub fn acond<'a, O, F>(boolean: bool, parser: F) -> impl Fn(&Span<'a>) -> ParserResult<'a, O>
where
    F: Fn(&Span<'a>) -> ParserResult<'a, O>,
{
    move |input| match boolean {
        true => parser(input),
        false => Err(ParserError::InactiveParser.locate(input.start)),
    }
}

pub fn opt<'a, O, F>(parser: F) -> impl Fn(&Span<'a>) -> ParserResult<'a, Option<O>>
where
    F: Fn(&Span<'a>) -> ParserResult<'a, O>,
{
    move |input| match parser(input) {
        Ok((input, value)) => Ok((input, Some(value))),
        Err(_) => Ok((input.clone(), None)),
    }
}

pub fn separated_pair<'a, O1, O2, O3, F1, F2, F3>(
    parser1: F1,
    parser2: F2,
    parser3: F3,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O1, O3)>
where
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3>,
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

pub trait ParseString<O> {
    fn parse_string<'a>(&self, input: &'a str) -> ParserOutput<O>;
}

impl<O, F: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O>> ParseString<O> for F {
    fn parse_string<'b>(&self, input: &'b str) -> ParserOutput<O> {
        let tokens = tokenize(input);
        let input = create_span(&tokens);
        self(&input).to_output()
    }
} 