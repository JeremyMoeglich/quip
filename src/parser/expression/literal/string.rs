use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while_m_n},
    character::complete::{char, multispace1},
    combinator::{map, map_opt, map_res, value},
    multi::fold_many0,
    sequence::{delimited, preceded},
    IResult,
};

use crate::{
    ast::{Expression, FancyString, FancyStringFragment},
    parser::utils::Span,
};

// largely taken from here https://github.com/Geal/nom/blob/main/examples/string.rs
fn parse_unicode(input: Span) -> IResult<Span, char> {
    let parse_hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());
    let parse_delimited_hex = preceded(char('u'), delimited(char('{'), parse_hex, char('}')));
    let parse_u32 = map_res(parse_delimited_hex, |s: Span| {
        u32::from_str_radix(s.fragment(), 16)
    });
    map_opt(parse_u32, |c| std::char::from_u32(c))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringKind {
    Single,
    Double,
}

fn parse_escaped_char(kind: StringKind) -> impl Fn(Span) -> IResult<Span, char> {
    move |input: Span| {
        let string_kind_char: char = match kind {
            StringKind::Single => '\'',
            StringKind::Double => '"',
        };
        preceded(
            char('\\'),
            alt((
                parse_unicode,
                value('\n', char('n')),
                value('\r', char('r')),
                value('\t', char('t')),
                value('\u{08}', char('b')),
                value('\u{0C}', char('f')),
                value('\\', char('\\')),
                value('/', char('/')),
                value('{', char('{')), // for fancy strings
                value(string_kind_char, char(string_kind_char)),
            )),
        )(input)
    }
}

fn parse_escaped_whitespace(input: Span) -> IResult<Span, Span> {
    preceded(char('\\'), multispace1)(input)
}

fn parse_basic_string(kind: StringKind) -> impl Fn(Span) -> IResult<Span, &str> {
    move |input: Span| {
        let (input, value) = is_not(
            format!(
                "{}\\{{",
                match kind {
                    StringKind::Single => "'",
                    StringKind::Double => "\"",
                }
            )
            .as_str(),
        )(input)?;

        if value.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::IsNot,
            )));
        } else {
            return Ok((input, &value));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum StringFragment {
    LiteralString(String),
    EscapedChar(char),
    EscapedWS,
    Expression(Expression),
    FormatPlaceholder,
}

fn parse_fragment(kind: StringKind) -> impl Fn(Span) -> IResult<Span, StringFragment> {
    move |input: Span| {
        alt((
            map(parse_escaped_char(kind), StringFragment::EscapedChar),
            map(parse_string_expression, StringFragment::Expression),
            map(tag("{}"), |_| StringFragment::FormatPlaceholder),
            map(parse_basic_string(kind), |s| {
                StringFragment::LiteralString(s.to_string())
            }),
            map(parse_escaped_whitespace, |_| StringFragment::EscapedWS),
        ))(input)
    }
}

fn parse_string_expression(input: Span) -> IResult<Span, Expression> {
    delimited(
        char('{'),
        crate::parser::expression::parse_expression,
        char('}'),
    )(input)
}

fn parse_normal_string(input: Span) -> IResult<Span, FancyString> {
    let string_content = |kind: StringKind| {
        fold_many0(
            parse_fragment(kind),
            Vec::new,
            |mut acc: FancyString, fragment| {
                let add_string = |acc: &mut FancyString, string: String| {
                    if let Some(FancyStringFragment::LiteralString(s)) = acc.last_mut() {
                        *s = format!("{}{}", s, string);
                    } else {
                        acc.push(FancyStringFragment::LiteralString(string));
                    }
                };
                match fragment {
                    StringFragment::LiteralString(s) => add_string(&mut acc, s.to_string()),
                    StringFragment::EscapedChar(c) => add_string(&mut acc, c.to_string()),
                    StringFragment::EscapedWS => {}
                    StringFragment::Expression(e) => acc.push(FancyStringFragment::Expression(e)),
                    StringFragment::FormatPlaceholder => {
                        acc.push(FancyStringFragment::FormatPlaceholder)
                    }
                };
                acc
            },
        )
    };
    alt((
        delimited(char('"'), string_content(StringKind::Double), char('"')),
        delimited(char('\''), string_content(StringKind::Single), char('\'')),
    ))(input)
}

pub fn parse_string(input: Span) -> IResult<Span, FancyString> {
    parse_normal_string(input) // TODO: add raw string support
}
