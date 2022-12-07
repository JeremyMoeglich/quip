use std::fmt::Debug;

use crate::fst::{Ident, Space, SpacePart};

use self::parser::{ParseErrorData, ParseResult, Parser, RecoveryFunc, TokenSlice};

use super::lexer::{LocatedToken, Token, TokenKind};

mod flatten;
mod parser;

pub fn not_it<'a, T: Debug + Clone>(
    error_token: Option<&'a LocatedToken<'a>>,
    valid_start: bool,
    recovery: RecoveryFunc<'a, T>,
    expected: Vec<TokenKind>,
) -> ParseResult<'a, T> {
    Err(ParseErrorData::new(
        error_token,
        valid_start,
        recovery,
        expected,
    ))
}

pub fn slice_next(input: TokenSlice) -> ParseResult<LocatedToken> {
    if let Some((head, tail)) = input.split_first() {
        ParseResult::Ok((tail, head.clone()))
    } else {
        Err(ParseErrorData::new(
            None,
            false,
            Box::new(|| (input, LocatedToken::new(Token::Error, Default::default()))),
            vec![],
        ))
    }
}

pub fn token<'a>(kind: TokenKind) -> impl Fn(TokenSlice<'a>) -> ParseResult<'a, LocatedToken<'a>> {
    move |input: TokenSlice<'a>| {
        let (input, token) = slice_next(input)?;
        if token.kind() == kind {
            Ok((input, token))
        } else {
            not_it(
                Some(&input[0]),
                false,
                Box::new(move || (input, token.clone())),
                vec![kind],
            )
        }
    }
}

pub fn tokens<'a>(
    kinds: &'a [TokenKind],
) -> impl FnMut(TokenSlice<'a>) -> ParseResult<'a, LocatedToken<'a>> {
    move |input: TokenSlice| {
        let (input, token) = slice_next(input)?;
        if kinds.contains(&token.kind()) {
            Ok((input, token))
        } else {
            not_it(
                Some(&input[0]),
                false,
                Box::new(move || (input, token.clone())),
                kinds.to_vec(),
            )
        }
    }
}

pub fn parse_ident<'a>(input: TokenSlice<'a>) -> ParseResult<'a, Ident> {
    match token(TokenKind::Ident)(input) {
        Ok((input, token)) => Ok((input, token.string())),
        Err(e) => not_it(
            e.error_token,
            false,
            map_recovery(e.recovery, |v| v.string()),
            e.expected,
        ),
    }
}

pub fn map_recovery<'a, T, U>(
    recovery: RecoveryFunc<'a, T>,
    f: impl FnOnce(T) -> U + 'a,
) -> RecoveryFunc<'a, U> {
    Box::new(move || {
        let (input, value) = recovery();
        (input, f(value))
    })
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

fn parse_space_part<'a>(input: TokenSlice<'a>) -> ParseResult<'a, SpacePart> {
    let err_func = |name: &str| format!("Failed to parse {} comment", name);
    match tokens(&[
        TokenKind::Whitespace,
        TokenKind::SingleLineComment,
        TokenKind::MultiLineComment,
    ])(input)
    {
        Ok((input, token)) => match token.token {
            Token::Whitespace(w) => Ok((input, SpacePart::Whitespace(w.to_string()))),
            Token::SingleLineComment(c) => Ok((
                input,
                SpacePart::SingleLineComment(
                    get_single_line_comment(c).expect(&err_func("single line")),
                ),
            )),
            Token::MultiLineComment(c) => Ok((
                input,
                SpacePart::MultiLineComment(
                    get_multi_line_comment(c).expect(&err_func("multi line")),
                ),
            )),
            _ => unreachable!(),
        },
        Err(e) => not_it(
            e.error_token,
            false,
            map_recovery(e.recovery, |v| SpacePart::Whitespace(" ".to_string())),
            e.expected,
        ),
    }
}

fn many<'a, T: Debug + Clone + 'a>(
    mut f: impl Parser<'a, T> + 'a,
    min_amount: usize,
) -> impl Parser<'a, Vec<T>> + 'a {
    move |input: TokenSlice<'a>| {
        let mut res = Vec::new();
        let mut input = input;
        let mut amount = 0;

        loop {
            match f.parse(input) {
                Ok((i, o)) => {
                    res.push(o);
                    input = i;
                    amount += 1;
                }
                Err(e) => {
                    if res.len() >= min_amount {
                        return Ok((input, res));
                    } else {
                        return not_it(
                            e.error_token,
                            amount > 0 || e.valid_start,
                            Box::new(move || (input, res)),
                            vec![TokenKind::Whitespace],
                        );
                    }
                }
            }
        }
    }
}

pub fn many0<'a, T: Debug + Clone + 'a>(
    f: impl Parser<'a, T> + 'a,
) -> impl Parser<'a, Vec<T>> + 'a {
    many(f, 0)
}

pub fn many1<'a, T: Debug + Clone + 'a>(
    f: impl Parser<'a, T> + 'a,
) -> impl Parser<'a, Vec<T>> + 'a {
    many(f, 1)
}

pub fn ws0<'a>(input: TokenSlice<'a>) -> ParseResult<'a, Space> {
    map(many0(parse_space_part), &Space::new).parse(input)
}

pub fn ws1<'a>(input: TokenSlice<'a>) -> ParseResult<'a, Space> {
    map(many1(parse_space_part), &Space::new).parse(input)
}

pub fn map<'a, T: Debug + Clone, U: Debug + Clone>(
    mut f: impl Parser<'a, T> + 'a,
    g: &'a impl Fn(T) -> U,
) -> impl Parser<'a, U> + 'a {
    move |input: TokenSlice<'a>| match f.parse(input) {
        Ok((input, value)) => Ok((input, g(value))),
        Err(e) => not_it(
            e.error_token,
            e.valid_start,
            map_recovery(e.recovery, g),
            e.expected,
        ),
    }
}

pub fn opt<'a, T: Debug + Clone + 'a>(
    f: impl Fn(TokenSlice<'a>) -> ParseResult<'a, T> + 'a,
) -> impl Fn(TokenSlice<'a>) -> ParseResult<'a, Option<T>> + 'a {
    move |input: TokenSlice<'a>| match f(input) {
        Ok((input, value)) => Ok((input, Some(value))),
        Err(e) => Ok((input, None)),
    }
}

pub fn force_eof<'a, T: Debug + Clone + 'a>(
    mut parser: impl FnMut(TokenSlice<'a>) -> ParseResult<'a, T>,
) -> impl FnMut(TokenSlice<'a>) -> ParseResult<'a, T> {
    move |input| {
        let (input, result) = parser(input)?;
        if input.is_empty() {
            Ok((input, result))
        } else {
            not_it(
                Some(&input[0]),
                true,
                Box::new(move || (input, result)),
                vec![],
            )
        }
    }
}

pub fn comma_separated<'a, T: Debug + Clone>(
    mut parser: impl Parser<'a, T> + 'a,
) -> impl Parser<'a, Vec<(T, Space, Option<Space>)>> + 'a {
    move |input: TokenSlice<'a>| {
        let mut last_arg = false;
        let (input, args) = many0(|input| {
            if last_arg {
                return not_it(
                    input.get(0),
                    true,
                    Box::new(move || (input, Vec::new())),
                    vec![TokenKind::Whitespace],
                );
            };
            let res = parser
                .chain(&ws0)
                .chain(&opt_token(TokenKind::Comma))
                .chain(&|input| {
                    let (input, comma) = opt_token(TokenKind::Comma)(input)?;
                    if let Some(_) = comma {
                        let (input, space) = ws0(input).unwrap();
                        Ok((input, Some(space)))
                    } else {
                        last_arg = true;
                        Ok((input, None))
                    }
                })
                .flattened();
            5
        })
        .parse(input)?;
        Ok((input, args))
    }
}

pub fn alt<'a, T: Debug + Clone>(
    mut parsers: Vec<impl Fn(TokenSlice<'a>) -> ParseResult<'a, T> + 'a>,
) -> impl Fn(TokenSlice<'a>) -> ParseResult<'a, T> + 'a {
    move |input: TokenSlice<'a>| {
        let mut best = None;
        let mut best_accuracy = 0.0;
        let mut best_expected = Vec::new();
        let mut best_error_token = None;
        let mut best_recovery = None;

        for parser in parsers.iter_mut() {
            match parser(input) {
                Ok((input, result)) => return Ok((input, result)),
                Err(e) => {
                    if e.accuracy > best_accuracy {
                        best_accuracy = e.accuracy;
                        best_expected = e.expected;
                        best_error_token = e.error_token;
                        best_recovery = e.recovery;
                    }
                }
                Incomplete => return incomplete(),
            }
        }

        not_it(
            best_error_token,
            best_accuracy,
            best_recovery.unwrap_or(Box::new(move || (input, best))),
            best_expected,
        )
    }
}
