#![feature(closure_lifetime_binder)]
pub mod lexer;

#[macro_use]
mod logs;

use enumset::EnumSet;
use fst::{Location, SourceSpan};
pub use lexer::*;
use logos::Logos;
use proc_macros::{generate_all_alt_impls, generate_all_tuple_impls};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Span<'a> {
    pub tokens: &'a [LocatedToken<'a>],
    pub start: Location,
}

#[derive(Debug, Clone, Copy)]
pub struct LocatedToken<'a> {
    pub source_span: SourceSpan,
    pub text: &'a str,
    pub token: Token<'a>,
}

impl<'a> LocatedToken<'a> {
    #[inline]
    pub fn kind(&self) -> TokenKind {
        self.token.kind()
    }
}

pub trait Delocate<'a> {
    fn delocate(&self) -> Option<Token<'a>>;
}

impl<'a> Delocate<'a> for Option<LocatedToken<'a>> {
    #[inline]
    fn delocate(&self) -> Option<Token<'a>> {
        self.as_ref().map(|t| t.token)
    }
}

pub trait AsTokenError<'a> {
    fn as_parser_error(
        &self,
        expected: EnumSet<TokenKind>,
        source_span: SourceSpan,
    ) -> LocatedParserError;
}

impl<'a> AsTokenError<'a> for Option<LocatedToken<'a>> {
    #[inline]
    fn as_parser_error(
        &self,
        expected: EnumSet<TokenKind>,
        source_span: SourceSpan,
    ) -> LocatedParserError {
        match self {
            Some(token) => ParserError::UnexpectedToken(Some(token.kind()), expected).locate({
                debug_assert!(source_span == token.source_span);
                source_span
            }),
            None => ParserError::UnexpectedToken(None, expected).locate(source_span),
        }
    }
}

#[inline]
pub fn tokenize<'a>(source: &'a str) -> Vec<LocatedToken<'a>> {
    let mut iter = Token::lexer(source);
    let mut tokens = Vec::new();
    let mut location = Location {
        column: 0,
        line: 0,
        index: 0,
    };
    while let Some(token) = iter.next() {
        let token = token.unwrap_or(Token::Error);
        let range = iter.span();
        let text = &source[range];
        let new_lines = text.chars().filter(|c| *c == '\n').count();
        let end_column = if new_lines == 0 {
            location.column + text.len()
        } else {
            text.lines().last().unwrap_or("").len()
        };
        let end_location = Location {
            column: end_column,
            line: location.line + new_lines,
            index: location.index + text.len(),
        };
        let located_token = LocatedToken {
            source_span: SourceSpan {
                start: location,
                end: end_location,
            },
            text,
            token,
        };
        tokens.push(located_token);
        location = end_location;
    }
    #[cfg(feature = "log")]
    log!(
        "Tokens: {:?}",
        tokens.iter().map(|t| t.token.kind()).collect::<Vec<_>>()
    );
    tokens
}

#[inline]
pub fn create_span<'a>(tokens: &'a [LocatedToken<'a>]) -> Span<'a> {
    let start = Location {
        column: 0,
        line: 0,
        index: 0,
    };
    Span { start, tokens }
}

pub trait TokensLength {
    fn len(&self) -> usize;
    fn new_lines(&self) -> usize;
    fn start_column(&self) -> usize;
}

impl TokensLength for LocatedToken<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.text.len()
    }
    #[inline]
    fn new_lines(&self) -> usize {
        self.text.chars().filter(|c| *c == '\n').count()
    }
    #[inline]
    fn start_column(&self) -> usize {
        let final_line = self.text.lines().last().unwrap_or("");
        if self.text.lines().count() == 1 {
            self.source_span.start.column + final_line.len()
        } else {
            final_line.len()
        }
    }
}

impl TokensLength for Span<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.tokens.iter().map(|t| t.len()).sum()
    }
    #[inline]
    fn new_lines(&self) -> usize {
        self.tokens.iter().map(|t| t.new_lines()).sum()
    }
    #[inline]
    fn start_column(&self) -> usize {
        self.tokens
            .last()
            .map(|t| t.start_column())
            .unwrap_or(self.start.column)
    }
}

#[derive(Error, Debug, Clone, Copy)]
pub enum ParserError {
    #[error("Unexpected token {0:?}, expected one of {}", format_enum_set(&.1))]
    /// Got, Expected
    UnexpectedToken(Option<TokenKind>, EnumSet<TokenKind>), // Got None is EndOfInput
}

fn format_enum_set(set: &EnumSet<TokenKind>) -> String {
    let elements: Vec<_> = set.iter().map(|token| format!("{:?}", token)).collect();
    elements.join(", ")
}

impl ParserError {
    #[inline]
    pub fn locate(self, source_span: SourceSpan) -> LocatedParserError {
        log!("{}: {}", source_span.start, self);
        LocatedParserError::new(self, source_span)
    }
}

#[derive(Error, Debug, Clone, Copy)]
#[error("{error} at {source_span}")]
pub struct LocatedParserError {
    pub error: ParserError,
    pub source_span: SourceSpan,
}

impl LocatedParserError {
    #[inline]
    pub fn new(error: ParserError, source_span: SourceSpan) -> Self {
        LocatedParserError { error, source_span }
    }
    #[inline]
    pub fn map_error<F: Fn(ParserError) -> ParserError>(self, wrapper: F) -> Self {
        Self {
            error: wrapper(self.error),
            source_span: self.source_span,
        }
    }
    #[inline]
    pub fn map_source_span<F: Fn(SourceSpan) -> SourceSpan>(self, wrapper: F) -> Self {
        Self {
            error: self.error,
            source_span: wrapper(self.source_span),
        }
    }
    #[inline]
    pub fn better_than(&self, other: &LocatedParserError) -> bool {
        use ParserError::*;

        match (&self.error, &other.error) {
            (UnexpectedToken(_, _), UnexpectedToken(_, _)) => {
                self.source_span.start > other.source_span.start
            }
        }
    }

    /// Accumulates two errors, returning the better one.
    /// If the errors are at the same location, the error with the most specific error is returned.
    /// Which error is passed first is only relevant if both errors are equally important.
    /// Then the first error is returned.
    #[inline]
    pub fn accumulate(&self, other: Self) -> Self {
        if other.better_than(self) {
            other
        } else if self.source_span == other.source_span {
            match (self.error.clone(), other.error) {
                (
                    ParserError::UnexpectedToken(got1, expected1),
                    ParserError::UnexpectedToken(got2, expected2),
                ) => {
                    if got1 != got2 {
                        println!("Expected 1: {:?}", expected1);
                        println!("Expected 2: {:?}", expected2);
                        unreachable!(
                            "Got two different tokens at the same location: {:?} {:?} at {}",
                            got1, got2, other.source_span
                        )
                    }
                    ParserError::UnexpectedToken(got1, expected1 | expected2)
                        .locate(other.source_span)
                } //_ => self.clone(),
            }
        } else {
            self.clone()
        }
    }
}

pub type ParserOutput<O> = Result<O, LocatedParserError>;
pub type ParserResult<'a, O> = Result<SafeParserResult<'a, O>, LocatedParserError>;
pub type SafeParserResult<'a, O> = (Span<'a>, O);

pub trait ParserResultTrait<'a, O> {
    fn to_output(self) -> ParserOutput<O>;
}

impl<'a, O> ParserResultTrait<'a, O> for ParserResult<'a, O> {
    #[inline]
    fn to_output(self) -> ParserOutput<O> {
        self.map(|(_, o)| o)
    }
}

impl<'a, O> ParserResultTrait<'a, O> for SafeParserResult<'a, O> {
    #[inline]
    fn to_output(self) -> ParserOutput<O> {
        Ok(self.1)
    }
}

pub trait IntoParserResult<'a, O> {
    fn into_parser_result(self) -> ParserResult<'a, O>;
}

impl<'a, O> IntoParserResult<'a, O> for SafeParserResult<'a, O> {
    #[inline]
    fn into_parser_result(self) -> ParserResult<'a, O> {
        Ok(self)
    }
}

impl<'a, O> IntoParserResult<'a, O> for ParserResult<'a, O> {
    #[inline]
    fn into_parser_result(self) -> ParserResult<'a, O> {
        self
    }
}

impl<'a> Span<'a> {
    #[inline]
    pub fn new(code: &'a [LocatedToken], start: Location) -> Self {
        Self {
            tokens: code,
            start,
        }
    }
    #[inline]
    pub fn first_token_span(&self) -> SourceSpan {
        match self.tokens.first() {
            Some(token) => token.source_span,
            None => SourceSpan {
                start: self.start,
                end: self.start,
            },
        }
    }

    #[inline]
    pub fn take_token(&self) -> SafeParserResult<'a, (Option<LocatedToken<'a>>, SourceSpan)> {
        if self.tokens.is_empty() {
            return (
                *self,
                (
                    None,
                    SourceSpan {
                        start: self.start,
                        end: self.start,
                    },
                ),
            );
        }

        debug_assert!(
            self.tokens.get(0).map(|t| t.source_span) != self.tokens.get(1).map(|t| t.source_span)
        );
        let token = self.tokens[0];
        debug_assert!(token.source_span.start.index == self.start.index);
        debug_assert!(token.source_span.start.index != token.source_span.end.index);

        let rest = Span {
            tokens: &self.tokens[1..],
            start: token.source_span.end,
        };

        (rest, (Some(token), token.source_span))
    }
    #[inline]
    pub fn end(&self) -> Location {
        match self.tokens.last() {
            Some(token) => token.source_span.end,
            None => self.start.clone(),
        }
    }
}

#[inline]
/// Parses zero or more occurrences of the parser.
/// Returns a vector of the parsed values.
pub fn many0<'a, O>(
    // a safe parser does not make sense here since that is just an infinite loop
    parser: impl Fn(Span<'a>) -> ParserResult<'a, O>,
) -> impl Fn(Span<'a>) -> SafeParserResult<'a, Vec<O>> {
    move |input: Span<'a>| {
        let mut input = input.clone();
        let mut output = vec![];
        loop {
            match parser(input) {
                Ok((rest, o)) => {
                    input = rest;
                    output.push(o);
                }
                Err(_) => break,
            }
        }
        (input, output)
    }
}

#[inline]
/// Parses zero or more occurrences of the parser.
/// Returning a span over the parsed tokens.
pub fn many0_span<'a, T>(
    parser: impl Fn(Span<'a>) -> ParserResult<'a, T>,
) -> impl Fn(Span<'a>) -> SafeParserResult<'a, Span<'a>> {
    move |input: Span<'a>| {
        let mut new_input = input.clone();
        loop {
            match parser(new_input).into_parser_result() {
                Ok((rest, _)) => {
                    new_input = rest;
                }
                Err(_) => break,
            }
        }
        (
            new_input,
            Span {
                start: input.start,
                tokens: &input.tokens[..(input.tokens.len() - new_input.tokens.len())],
            },
        )
    }
}

#[inline]
pub fn aggressive_many0<'a, O, PR: IntoParserResult<'a, O>>(
    parser: impl Fn(Span<'a>) -> PR,
) -> impl Fn(Span<'a>) -> ParserResult<'a, Vec<O>> {
    move |input: Span<'a>| {
        let mut input = input.clone();
        let mut output = vec![];
        loop {
            match parser(input).into_parser_result() {
                Ok((rest, o)) => {
                    input = rest;
                    output.push(o);
                }
                Err(e) => {
                    if !input.tokens.is_empty() {
                        return Err(e);
                    } else {
                        break;
                    }
                }
            }
        }
        Ok((input, output))
    }
}

#[inline]
pub fn many1<'a, O, PR: IntoParserResult<'a, O>>(
    parser: impl Fn(Span<'a>) -> PR,
) -> impl Fn(Span<'a>) -> ParserResult<'a, Vec<O>> {
    move |input: Span<'a>| {
        let mut input = input.clone();
        let mut output = vec![];
        loop {
            match parser(input).into_parser_result() {
                Ok((rest, o)) => {
                    input = rest;
                    output.push(o);
                }
                Err(e) => {
                    if output.is_empty() {
                        return Err(e);
                    } else {
                        break;
                    }
                }
            }
        }
        Ok((input, output))
    }
}

#[inline]
pub fn many1_span<'a, T>(
    parser: impl Fn(Span<'a>) -> ParserResult<'a, T>,
) -> impl Fn(Span<'a>) -> ParserResult<'a, Span<'a>> {
    move |input: Span<'a>| {
        let mut new_input = input.clone();
        let mut matched = false;

        loop {
            match parser(new_input).into_parser_result() {
                Ok((rest, _)) => {
                    new_input = rest;
                    matched = true;
                }
                Err(e) => {
                    // If no match was found, return the error.
                    if !matched {
                        return Err(e);
                    }
                    // Return a Span that covers the range of successfully parsed tokens.
                    return Ok((
                        new_input,
                        Span {
                            start: input.start,
                            tokens: &input.tokens[..(input.tokens.len() - new_input.tokens.len())],
                        },
                    ));
                }
            }
        }
    }
}



#[macro_export]
macro_rules! extract_token_data {
    ($located_token:expr, $token_kind:ident) => {
        match $located_token.token {
            Token::$token_kind(data) => Some(data),
            _ => None,
        }
    };
}

pub trait Alt<'a, O> {
    fn alt<'b>(&'b self) -> impl Fn(Span<'a>) -> ParserResult<'a, O> + 'b;
}

generate_all_alt_impls!(24);

pub trait Tuple<'a, O> {
    fn tuple<'b>(&'b self) -> impl Fn(Span<'a>) -> ParserResult<'a, O> + 'b;
}

generate_all_tuple_impls!(24);

pub fn any_of(
    token_kinds: EnumSet<TokenKind>,
) -> impl for<'a> Fn(Span<'a>) -> ParserResult<'a, LocatedToken<'a>> {
    for<'b> move |input: Span<'b>| -> ParserResult<'b, LocatedToken<'b>> {
        let (input, (token, source_span)) = input.take_token();
        match token {
            Some(token) => {
                if token_kinds.contains(token.kind()) {
                    Ok((input, token))
                } else {
                    Err(
                        ParserError::UnexpectedToken(Some(token.kind()), token_kinds)
                            .locate(token.source_span),
                    )
                }
            }
            None => Err(ParserError::UnexpectedToken(None, token_kinds).locate(source_span)),
        }
    }
}
pub trait MapParser<'a, O> {
    fn map<O2, F: Fn(O) -> O2>(self, wrapper: F) -> impl Fn(Span<'a>) -> ParserResult<'a, O2>;
    fn map_err<F: Fn(ParserError) -> ParserError>(
        self,
        wrapper: F,
    ) -> impl Fn(Span<'a>) -> ParserResult<'a, O>;
}

impl<'a, O, P: Fn(Span<'a>) -> ParserResult<'a, O>> MapParser<'a, O> for P {
    #[inline]
    fn map<O2, F: Fn(O) -> O2>(self, wrapper: F) -> impl Fn(Span<'a>) -> ParserResult<'a, O2> {
        move |span: Span<'a>| {
            let (input, output) = self(span)?;
            Ok((input, wrapper(output)))
        }
    }
    #[inline]
    fn map_err<F: Fn(ParserError) -> ParserError>(
        self,
        wrapper: F,
    ) -> impl Fn(Span<'a>) -> ParserResult<'a, O> {
        move |span: Span<'a>| {
            let result = self(span);
            match result {
                Ok((input, output)) => Ok((input, output)),
                Err(e) => Err(e.map_error(&wrapper)),
            }
        }
    }
}

pub trait SafeMapParser<'a, O> {
    fn map<O2, F: Fn(O) -> O2>(self, wrapper: F) -> impl Fn(Span<'a>) -> SafeParserResult<'a, O2>;
}

impl<'a, O, P: Fn(Span<'a>) -> SafeParserResult<'a, O>> SafeMapParser<'a, O> for P {
    #[inline]
    fn map<O2, F: Fn(O) -> O2>(self, wrapper: F) -> impl Fn(Span<'a>) -> SafeParserResult<'a, O2> {
        move |span: Span<'a>| {
            let (input, output) = self(span);
            (input, wrapper(output))
        }
    }
}
