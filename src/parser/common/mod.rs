use std::{collections::HashSet, fmt::Debug};

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
            HashSet::new(),
        ))
    }
}

pub fn token(kind: TokenKind) -> impl Parser<Output = LocatedToken> {
    for<'a> move |input: TokenSlice<'a>| -> ParseResult<'a, LocatedToken> {
        let (input, token) = slice_next(input)?;
        if token.kind() == kind {
            Ok((input, token))
        } else {
            Err(not_it(
                Some(&input[0]),
                false,
                Box::new(move || (input, token.clone())),
                {
                    let mut set = HashSet::new();
                    set.insert(kind);
                    set
                },
            ))
        }
    }
}

pub fn tokens<'a>(
    kinds: &'a [TokenKind],
) -> impl for<'b> Fn(TokenSlice<'b>) -> ParseResult<'b, LocatedToken> + 'a {
    move |input| {
        let (input, token) = slice_next(input)?;
        if kinds.contains(&token.kind()) {
            Ok((input, token))
        } else {
            Err(not_it(
                Some(&input[0]),
                false,
                Box::new(move || (input, token.clone())),
                {
                    let mut set = HashSet::new();
                    set.extend(kinds);
                    set
                },
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
    f: impl Fn(T) -> U + 'a,
) -> RecoveryFunc<'a, U> {
    Box::new(move || {
        let (input, value) = recovery();
        (input, f(value))
    })
}

pub fn opt_token<'a>(
    kind: TokenKind,
) -> impl Fn(TokenSlice<'a>) -> ParseResult<'a, Option<LocatedToken>> {
    move |input: TokenSlice<'a>| {
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
                    get_single_line_comment(&c).expect(&err_func("single line")),
                ),
            )),
            Token::MultiLineComment(c) => Ok((
                input,
                SpacePart::MultiLineComment(
                    get_multi_line_comment(&c).expect(&err_func("multi line")),
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

type ManyParserCheckOption<T, U> = Option<Box<dyn Fn(Option<U>, T) -> (U, bool)>>;

struct ManyParser<T: Debug + Clone, P: Parser<Output = T>, U = ()> {
    parser: P,
    min_amount: usize,
    check: ManyParserCheckOption<T, U>,
}

impl<T: Clone + Debug, P: Parser<Output = T>, U> Parser for ManyParser<T, P, U> {
    type Output = Vec<T>;
    fn parse<'b>(&self, mut input: ParserInput<'b>) -> ParseResult<'b, Vec<T>> {
        let mut amount = 0;
        let mut result = Vec::new();
        let mut check_value = None;
        let mut final_error = None;
        loop {
            match self.parser.parse(input) {
                Ok((new_input, value)) => {
                    match self.check {
                        Some(ref check) => {
                            let (new_check_value, should_continue) =
                                check(check_value, value.clone());
                            check_value = Some(new_check_value);
                            if !should_continue {
                                break;
                            }
                        }
                        None => (),
                    }
                    input = new_input;
                    result.push(value);
                    amount += 1;
                }
                Err(e) => {
                    final_error = Some(e);
                    break;
                }
            }
        }
        if amount >= self.min_amount {
            Ok((input, result))
        } else {
            let final_error = final_error.unwrap();
            Err(not_it(
                final_error.error_token,
                false,
                map_recovery(final_error.recovery, |r| {
                    result.push(r);
                    result
                }),
                final_error.expected,
            ))
        }
    }
}

impl<T: Clone + Debug, P: Parser<Output = T>, U> ManyParser<T, P, U> {
    fn new(parser: P, min_amount: usize, check: ManyParserCheckOption<T, U>) -> Self {
        Self {
            parser,
            min_amount,
            check,
        }
    }
    fn check(self, check: impl Fn(Option<U>, T) -> (U, bool)) -> Self {
        Self {
            parser: self.parser,
            min_amount: self.min_amount,
            check: Some(Box::new(move |u, t| check(u, t)) as _),
        }
    }
}

fn many<'p, 's, T: Debug + Clone, P: Parser<Output = T>, U>(
    f: P,
    min_amount: usize,
) -> ManyParser<T, P, U> {
    ManyParser::new(f, min_amount, None)
}

pub fn many0<'p, 's, T: Debug + Clone, P: Parser<Output = T>>(f: P) -> ManyParser<T, P, ()> {
    many(f, 0)
}

pub fn many1<'p, 's, T: Debug + Clone, P: Parser<Output = T>>(f: P) -> ManyParser<T, P, ()> {
    many(f, 1)
}

pub fn checked_many0<'p, 's, T: Debug + Clone, P: Parser<Output = T>, U>(
    f: P,
) -> ManyParser<T, P, U> {
    many(f, 0)
}

pub fn checked_many1<'p, 's, T: Debug + Clone, P: Parser<Output = T>, U>(
    f: P,
) -> ManyParser<T, P, U> {
    many(f, 1)
}

pub fn ws0<'a>(input: TokenSlice<'a>) -> ParseResult<'a, Space> {
    many0(parse_space_part).map_result(&Space::new).parse(input)
}

pub fn ws1<'a>(input: TokenSlice<'a>) -> ParseResult<'a, Space> {
    many1(parse_space_part).map_result(&Space::new).parse(input)
}

pub fn opt<'a, T: Debug + Clone + 'a>(
    f: impl Fn(TokenSlice<'a>) -> ParseResult<'a, T> + 'a,
) -> impl Fn(TokenSlice<'a>) -> ParseResult<'a, Option<T>> + 'a {
    move |input: TokenSlice<'a>| match f(input) {
        Ok((input, value)) => Ok((input, Some(value))),
        Err(e) => Ok((input, None)),
    }
}

pub fn force_eof<'s, T: Debug + Clone, P: Parser<Output = T> + 's>(
    parser: P,
) -> Box<dyn for<'a> Fn(TokenSlice<'a>) -> ParseResult<'a, T> + 's> {
    Box::new(move |input| {
        let (input, result) = parser.parse(input)?;
        if input.is_empty() {
            Ok((input, result))
        } else {
            Err(not_it(
                Some(&input[0]),
                true,
                Box::new(move || (input, result)),
                HashSet::new(),
            ))
        }
    })
}

pub fn comma_separated<T: Debug + Clone + BaseElement>(
    parser: impl Parser<Output = T>,
) -> Box<dyn for<'a> Fn(ParserInput<'a>) -> ParseResult<'a, Vec<(T, Space, Option<Space>)>>> {
    Box::new(move |input| {
        checked_many0(
            parser
                .chain(ws0)
                .chain(opt_token(TokenKind::Comma))
                .chain(|input| {
                    let (input, comma) = opt_token(TokenKind::Comma)(input).unwrap();
                    if let Some(_) = comma {
                        let (input, space) = ws0(input).unwrap();
                        Ok((input, Some(space)))
                    } else {
                        Ok((input, None))
                    }
                })
                .flattened()
                .map_result(&|(value, space1, comma, space2)| (value, space1, None)),
        )
        .check(|had_comma, (_, _, space2)| match had_comma {
            Some(had_comma) => match had_comma {
                true => match space2 {
                    Some(_) => (true, true),
                    None => (false, true),
                },
                false => match space2 {
                    Some(_) => (true, false),
                    None => (false, false),
                },
            },
            None => (true, true),
        })
        .parse(input)
    })
}
