use std::fmt::Debug;

use crate::fst::{Ident, Space, SpacePart};

use super::{
    core::{
        flatten::BaseElement, not_it, ParseErrorData, ParseResult, Parser, ParserInput,
        RecoveryFunc, TokenSlice,
    },
    lexer::{LocatedToken, Token, TokenKind},
};

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

pub fn token<'a>(kind: TokenKind) -> impl Parser<'a, LocatedToken<'a>> {
    move |input: TokenSlice<'a>| {
        let (input, token) = slice_next(input)?;
        if token.kind() == kind {
            Ok((input, token))
        } else {
            Err(not_it(
                Some(&input[0]),
                false,
                Box::new(move || (input, token.clone())),
                vec![kind],
            ))
        }
    }
}

pub fn tokens<'a>(
    kinds: &'a [TokenKind],
) -> impl Fn(TokenSlice<'a>) -> ParseResult<'a, LocatedToken<'a>> {
    move |input: TokenSlice| {
        let (input, token) = slice_next(input)?;
        if kinds.contains(&token.kind()) {
            Ok((input, token))
        } else {
            Err(not_it(
                Some(&input[0]),
                false,
                Box::new(move || (input, token.clone())),
                kinds.to_vec(),
            ))
        }
    }
}

pub fn parse_ident<'a>(input: TokenSlice<'a>) -> ParseResult<'a, Ident> {
    match token(TokenKind::Ident).parse(input) {
        Ok((input, token)) => Ok((input, token.string())),
        Err(e) => Err(not_it(
            e.error_token,
            false,
            map_recovery(e.recovery, |v| v.string()),
            e.expected,
        )),
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
        if let Ok((input, token)) = token(kind).parse(input) {
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
        Err(e) => Err(not_it(
            e.error_token,
            false,
            map_recovery(e.recovery, |v| SpacePart::Whitespace(" ".to_string())),
            e.expected,
        )),
    }
}

type ManyParserOption<T, U> = Option<Box<dyn Fn(U, T) -> (U, bool)>>;

struct ManyParser<'a, T: Debug + Clone, U = ()> {
    parser: Box<dyn Fn(ParserInput<'a>) -> ParseResult<'a, T>>,
    check: ManyParserOption<T, U>,
}

impl<'a, T: Clone + Debug, U> Parser<'a, T> for ManyParser<'a, T, U> {
    fn parse(&self, input: ParserInput<'a>) -> ParseResult<'a, T> {
        self.parser.parse(input)
    }
}

impl<'a, T: Clone + Debug, U> ManyParser<'a, T, U> {
    fn new(parser: impl Parser<'a, T> + 'a, check: ManyParserOption<T, U>) -> Self {
        Self {
            parser: Box::new(move |input| parser.parse(input)),
            check: check.map(|f| Box::new(move |u, t| f(u, t)) as _),
        }
    }
    fn check(self, check: impl Fn(U, T) -> (U, bool) + 'a) -> Self {
        Self {
            parser: self.parser,
            check: Some(Box::new(move |u, t| check(u, t)) as _),
        }
    }
}

fn many<'a, T: Debug + Clone + 'a, U>(
    f: impl Parser<'a, T> + 'a,
    min_amount: usize,
) -> ManyParser<'a, Vec<T>, U> {
    let parser = move |input: TokenSlice<'a>| {
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
                        return Err(not_it(
                            e.error_token,
                            amount > 0 || e.valid_start,
                            Box::new(move || (input, res)),
                            vec![TokenKind::Whitespace],
                        ));
                    }
                }
            }
        }
    };
    let option: Option<_> = None;
    ManyParser::new(parser, option)
}

pub fn many0<'a, T: Debug + Clone + 'a>(f: impl Parser<'a, T> + 'a) -> ManyParser<'a, Vec<T>> {
    many(f, 0)
}

pub fn many1<'a, T: Debug + Clone + 'a>(f: impl Parser<'a, T> + 'a) -> ManyParser<'a, Vec<T>> {
    many(f, 1)
}

pub fn ws0<'a>(input: TokenSlice<'a>) -> ParseResult<'a, Space> {
    map(many0(parse_space_part), &Space::new).parse(input)
}

pub fn ws1<'a>(input: TokenSlice<'a>) -> ParseResult<'a, Space> {
    map(many1(parse_space_part), &Space::new).parse(input)
}

pub fn map<'a, T: Debug + Clone, U: Debug + Clone>(
    f: impl Parser<'a, T> + 'a,
    g: &'a impl Fn(T) -> U,
) -> impl Parser<'a, U> + 'a {
    move |input: TokenSlice<'a>| match f.parse(input) {
        Ok((input, value)) => Ok((input, g(value))),
        Err(e) => Err(not_it(
            e.error_token,
            e.valid_start,
            map_recovery(e.recovery, g),
            e.expected,
        )),
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
    parser: impl Fn(TokenSlice<'a>) -> ParseResult<'a, T>,
) -> impl Fn(TokenSlice<'a>) -> ParseResult<'a, T> {
    move |input| {
        let (input, result) = parser(input)?;
        if input.is_empty() {
            Ok((input, result))
        } else {
            Err(not_it(
                Some(&input[0]),
                true,
                Box::new(move || (input, result)),
                vec![],
            ))
        }
    }
}

pub fn comma_separated<'a, T: Debug + Clone + BaseElement + 'a>(
    parser: impl Parser<'a, T> + 'a,
) -> impl Parser<'a, Vec<(T, Space, Option<Space>)>> + 'a {
    let token_parser = opt_token(TokenKind::Comma);
    let segment_parser = parser
        .chain(&ws0)
        .chain(token_parser)
        .chain(&|input| {
            let (input, comma) = opt_token(TokenKind::Comma)(input).unwrap();
            if let Some(_) = comma {
                let (input, space) = ws0(input).unwrap();
                Ok((input, Some(space)))
            } else {
                Ok((input, None))
            }
        })
        .flattened()
        .map_result(&|(value, space1, comma, space2)| (value, space1, None));
    many0(segment_parser)
}

pub fn alt<'a, T: Debug + Clone>(
    parsers: Vec<impl Fn(TokenSlice<'a>) -> ParseResult<'a, T> + 'a>,
) -> impl Fn(TokenSlice<'a>) -> ParseResult<'a, T> + 'a {
    move |input: TokenSlice<'a>| {
        let mut current_err = None;

        for parser in parsers.iter() {
            match parser(input) {
                Ok((input, result)) => return Ok((input, result)),
                Err(e) => {
                    current_err = match current_err {
                        None => Some(e),
                        Some(current_err) => {
                            if e.error_token.is_some() && current_err.error_token.is_none() {
                                let exprected = current_err
                                    .expected
                                    .iter()
                                    .chain(e.expected.iter())
                                    .cloned()
                                    .collect();
                                let best = match current_err.valid_start {
                                    true => current_err,
                                    false => e,
                                };
                                Some(not_it(
                                    best.error_token,
                                    best.valid_start,
                                    best.recovery,
                                    exprected,
                                ))
                            } else {
                                Some(current_err)
                            }
                        }
                    }
                }
            }
        }

        Err(current_err.expect("No parsers provided to alt"))
    }
}
