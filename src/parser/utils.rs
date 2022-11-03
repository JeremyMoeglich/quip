#![allow(dead_code)]
extern crate nom;
extern crate nom_locate;

use nom::{
    branch::alt,
    bytes::{complete::is_not, complete::tag},
    character::complete::{multispace1, not_line_ending},
    multi::{many0, many1},
    sequence::delimited,
    IResult,
};

use nom_locate::LocatedSpan;

// Utils for parsing

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn new_span(input: &str) -> Span {
    Span::new_extra(input, ())
}

fn single_line_comment(input: Span) -> IResult<Span, Span> {
    let (input, _) = tag("//")(input)?;
    let (input, comment) = not_line_ending(input)?;
    Ok((input, comment))
}

fn multi_line_comment(input: Span) -> IResult<Span, Span> {
    let comment_start = "/*";
    let comment_end = "*/";
    let (input, _) = delimited(tag(comment_start), is_not(comment_end), tag(comment_end))(input)?;
    Ok((input, input))
}

fn comment(input: Span) -> IResult<Span, Span> {
    alt((single_line_comment, multi_line_comment))(input)
}

pub fn ws(input: Span) -> IResult<Span, Span> {
    let (input, _) = many0(alt((multispace1, comment)))(input)?;
    Ok((input, input))
}

pub fn ws1(input: Span) -> IResult<Span, Span> {
    let (input, _) = many1(alt((comment, multispace1)))(input)?;
    Ok((input, input))
}

pub fn vec_alt<'a, O, F>(mut parsers: Vec<F>) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O>
where
    F: FnMut(Span<'a>) -> IResult<Span<'a>, O>,
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

pub fn vec_tuple<'a, O, F>(mut parsers: Vec<F>) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, Vec<O>>
where
    F: FnMut(Span<'a>) -> IResult<Span<'a>, O>,
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
