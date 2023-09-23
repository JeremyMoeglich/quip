#![feature(prelude_import)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(closure_lifetime_binder)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::cmp::Ordering;
use ast::Location;
use lexer::{Token, TokenKind};
mod error_union {}
use logos::Logos;
use parser_proc::{generate_all_alt_impls, generate_all_tuple_impls};
use thiserror::Error;
pub struct Span<'a> {
    pub tokens: &'a [LocatedToken<'a>],
    pub start: Location,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for Span<'a> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Span",
            "tokens",
            &self.tokens,
            "start",
            &&self.start,
        )
    }
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for Span<'a> {
    #[inline]
    fn clone(&self) -> Span<'a> {
        Span {
            tokens: ::core::clone::Clone::clone(&self.tokens),
            start: ::core::clone::Clone::clone(&self.start),
        }
    }
}
pub struct LocatedToken<'a> {
    pub start: Location,
    pub text: &'a str,
    pub token: Token<'a>,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for LocatedToken<'a> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "LocatedToken",
            "start",
            &self.start,
            "text",
            &self.text,
            "token",
            &&self.token,
        )
    }
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for LocatedToken<'a> {
    #[inline]
    fn clone(&self) -> LocatedToken<'a> {
        LocatedToken {
            start: ::core::clone::Clone::clone(&self.start),
            text: ::core::clone::Clone::clone(&self.text),
            token: ::core::clone::Clone::clone(&self.token),
        }
    }
}
impl<'a> LocatedToken<'a> {
    pub fn kind(&self) -> TokenKind {
        self.token.kind()
    }
}
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
        let column = if new_lines == 0 {
            location.column + text.len()
        } else {
            text.lines().last().unwrap_or("").len()
        };
        let located_token = LocatedToken {
            start: location,
            text,
            token,
        };
        tokens.push(located_token);
        location = Location {
            column,
            line: location.line + new_lines,
            index: location.index + text.len(),
        };
    }
    tokens
}
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
    fn column(&self) -> usize;
}
impl TokensLength for LocatedToken<'_> {
    fn len(&self) -> usize {
        self.text.len()
    }
    fn new_lines(&self) -> usize {
        self.text.chars().filter(|c| *c == '\n').count()
    }
    fn column(&self) -> usize {
        let final_line = self.text.lines().last().unwrap_or("");
        if self.text.lines().count() == 1 {
            self.start.column + final_line.len()
        } else {
            final_line.len()
        }
    }
}
impl TokensLength for Span<'_> {
    fn len(&self) -> usize {
        self.tokens.iter().map(|t| t.len()).sum()
    }
    fn new_lines(&self) -> usize {
        self.tokens.iter().map(|t| t.new_lines()).sum()
    }
    fn column(&self) -> usize {
        self.tokens.last().map(|t| t.column()).unwrap_or(self.start.column)
    }
}
pub enum ParserError {
    #[error("Unexpected token {0:?}, expected one of {1:?}")]
    UnexpectedToken(TokenKind, Vec<TokenKind>),
    #[error("Unexpected end of input")]
    EndOfInput,
    #[error("Parser inactive")]
    InactiveParser,
}
#[allow(unused_qualifications)]
impl std::error::Error for ParserError {}
#[allow(unused_qualifications)]
impl std::fmt::Display for ParserError {
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            ParserError::UnexpectedToken(_0, _1) => {
                __formatter
                    .write_fmt(
                        format_args!(
                            "Unexpected token {0:?}, expected one of {1:?}", _0, _1
                        ),
                    )
            }
            ParserError::EndOfInput {} => {
                __formatter.write_fmt(format_args!("Unexpected end of input"))
            }
            ParserError::InactiveParser {} => {
                __formatter.write_fmt(format_args!("Parser inactive"))
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for ParserError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            ParserError::UnexpectedToken(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "UnexpectedToken",
                    __self_0,
                    &__self_1,
                )
            }
            ParserError::EndOfInput => ::core::fmt::Formatter::write_str(f, "EndOfInput"),
            ParserError::InactiveParser => {
                ::core::fmt::Formatter::write_str(f, "InactiveParser")
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for ParserError {
    #[inline]
    fn clone(&self) -> ParserError {
        match self {
            ParserError::UnexpectedToken(__self_0, __self_1) => {
                ParserError::UnexpectedToken(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
            ParserError::EndOfInput => ParserError::EndOfInput,
            ParserError::InactiveParser => ParserError::InactiveParser,
        }
    }
}
impl ParserError {
    pub fn locate(self, location: Location) -> LocatedParserError {
        LocatedParserError::new(self, location)
    }
}
#[error("{error} at {location}")]
pub struct LocatedParserError {
    pub error: ParserError,
    pub location: Location,
}
#[allow(unused_qualifications)]
impl std::error::Error for LocatedParserError {}
#[allow(unused_qualifications)]
impl std::fmt::Display for LocatedParserError {
    #[allow(clippy::used_underscore_binding)]
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_imports)]
        use thiserror::__private::{DisplayAsDisplay, PathAsDisplay};
        #[allow(unused_variables, deprecated)]
        let Self { error, location } = self;
        __formatter
            .write_fmt(
                format_args!("{0} at {1}", error.as_display(), location.as_display()),
            )
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for LocatedParserError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "LocatedParserError",
            "error",
            &self.error,
            "location",
            &&self.location,
        )
    }
}
impl LocatedParserError {
    pub fn new(error: ParserError, location: Location) -> Self {
        LocatedParserError {
            error,
            location,
        }
    }
    pub fn map_error<F: Fn(ParserError) -> ParserError>(self, wrapper: F) -> Self {
        Self {
            error: wrapper(self.error),
            location: self.location,
        }
    }
    pub fn map_location<F: Fn(Location) -> Location>(self, wrapper: F) -> Self {
        Self {
            error: self.error,
            location: wrapper(self.location),
        }
    }
    pub fn better_than(&self, other: &LocatedParserError) -> bool {
        use ParserError::*;
        match (&self.error, &other.error) {
            (EndOfInput, EndOfInput)
            | (UnexpectedToken(_, _), UnexpectedToken(_, _))
            | (InactiveParser, InactiveParser) => self.location > other.location,
            (EndOfInput, _) => true,
            (_, EndOfInput) => false,
            (UnexpectedToken(_, _), _) => true,
            (_, UnexpectedToken(_, _)) => false,
        }
    }
}
pub type ParserResult<'a, O> = Result<(Span<'a>, O), LocatedParserError>;
impl<'a> Span<'a> {
    pub fn new(code: &'a [LocatedToken], start: Location) -> Self {
        Self { tokens: code, start }
    }
    pub fn take_n_token(&self, n: usize) -> ParserResult<'a, Span<'a>> {
        if n == 0 {
            return Ok((
                Span {
                    tokens: &[],
                    start: self.start.clone(),
                },
                Span {
                    tokens: self.tokens,
                    start: self.start.clone(),
                },
            ));
        }
        if n > self.tokens.len() {
            return Err(LocatedParserError::new(ParserError::EndOfInput, self.end()));
        }
        let (chunk, rest) = self.tokens.split_at(n);
        let rest_start = Location {
            line: rest
                .first()
                .map(|t| t.start.line)
                .unwrap_or(
                    self.start.line + chunk.iter().map(|t| t.new_lines()).sum::<usize>(),
                ),
            column: rest
                .first()
                .map(|t| t.start.column)
                .unwrap_or({
                    if chunk.iter().map(|t| t.new_lines()).sum::<usize>() == 0 {
                        self.start.column + chunk.iter().map(|t| t.len()).sum::<usize>()
                    } else {
                        chunk.last().unwrap().column()
                    }
                }),
            index: rest
                .first()
                .map(|t| t.start.index)
                .unwrap_or(
                    self.start.index + chunk.iter().map(|t| t.len()).sum::<usize>(),
                ),
        };
        let chunk_span = Span::new(chunk, self.start.clone());
        let rest_span = Span::new(rest, rest_start.clone());
        Ok((rest_span, chunk_span))
    }
    pub fn take_tokens<const N: usize>(
        &self,
    ) -> ParserResult<'a, &'a [LocatedToken<'a>; N]> {
        let (rest, chunk) = self.take_n_token(N)?;
        let chunk = chunk.tokens;
        let chunk = chunk.try_into().unwrap();
        Ok((rest, chunk))
    }
    pub fn take_token(&self) -> ParserResult<'a, LocatedToken<'a>> {
        let (rest, chunk) = self.take_tokens::<1>()?;
        Ok((rest, chunk[0].clone()))
    }
    pub fn peek_first_kind(&self) -> Option<TokenKind> {
        match self.tokens.first() {
            Some(token) => Some(token.kind()),
            None => None,
        }
    }
    pub fn end(&self) -> Location {
        match self.tokens.last() {
            Some(token) => {
                let index = token.start.index + token.text.len();
                let line = token.start.line + token.text.lines().count() - 1;
                let final_line = token.text.lines().last().unwrap_or("");
                let column = if token.text.lines().count() == 1 {
                    token.start.column + final_line.len()
                } else {
                    final_line.len()
                };
                Location { index, line, column }
            }
            None => {
                Location {
                    index: 0,
                    line: 0,
                    column: 0,
                }
            }
        }
    }
}
pub fn take_while<'a, F: Fn(&LocatedToken<'a>) -> bool>(
    predicate: F,
) -> impl Fn(&Span<'a>) -> (Span<'a>, Span<'a>) {
    move |span: &Span<'a>| {
        let mut i = 0;
        while i < span.tokens.len() && predicate(&span.tokens[i]) {
            i += 1;
        }
        span.take_n_token(i).unwrap()
    }
}
pub fn many0<'a, O>(
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>> {
    move |input: &Span<'a>| {
        let mut input = input.clone();
        let mut output = ::alloc::vec::Vec::new();
        loop {
            match parser(&input) {
                Ok((rest, o)) => {
                    input = rest;
                    output.push(o);
                }
                Err(_) => break,
            }
        }
        Ok((input, output))
    }
}
pub fn many1<'a, O>(
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>> {
    move |input: &Span<'a>| {
        let mut input = input.clone();
        let mut output = ::alloc::vec::Vec::new();
        loop {
            match parser(&input) {
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
pub fn separated_list0<'a, O, O_>(
    separator: impl Fn(&Span<'a>) -> ParserResult<'a, O_>,
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>> {
    move |input: &Span<'a>| {
        let mut input = input.clone();
        let mut output = ::alloc::vec::Vec::new();
        loop {
            if let Ok((rest, o)) = parser(&input) {
                input = rest;
                output.push(o);
            } else {
                break;
            }
            if let Ok((rest, _)) = separator(&input) {
                input = rest;
            } else {
                break;
            }
        }
        Ok((input, output))
    }
}
pub fn separated_list1<'a, O, E>(
    separator: impl Fn(&Span<'a>) -> ParserResult<'a, ()>,
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>> {
    move |input: &Span<'a>| {
        let mut input = input.clone();
        let mut output = ::alloc::vec::Vec::new();
        match parser(&input) {
            Ok((rest, o)) => {
                input = rest;
                output.push(o);
            }
            Err(e) => return Err(e),
        }
        loop {
            if let Ok((rest, _)) = separator(&input) {
                input = rest;
            } else {
                break;
            }
            if let Ok((rest, o)) = parser(&input) {
                input = rest;
                output.push(o);
            } else {
                break;
            }
        }
        Ok((input, output))
    }
}
pub fn delimited<'a, O, O1_, O2_>(
    start: impl Fn(&Span<'a>) -> ParserResult<'a, O1_>,
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O>,
    end: impl Fn(&Span<'a>) -> ParserResult<'a, O2_>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, O> {
    move |input: &Span<'a>| {
        let mut input = input.clone();
        let (rest, _) = start(&input)?;
        input = rest;
        let (rest, o) = parser(&input)?;
        input = rest;
        let (rest, _) = end(&input)?;
        input = rest;
        Ok((input, o))
    }
}
pub fn preceded<'a, O1_, O>(
    first: impl Fn(&Span<'a>) -> ParserResult<'a, O1_>,
    second: impl Fn(&Span<'a>) -> ParserResult<'a, O>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, O> {
    move |input: &Span<'a>| {
        let mut input = input.clone();
        let (rest, _) = first(&input)?;
        input = rest;
        let (rest, o) = second(&input)?;
        input = rest;
        Ok((input, o))
    }
}
pub trait Alt<'a, O> {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O> + 'b;
}
impl<'a, Out, F0: Fn(&Span<'a>) -> ParserResult<'a, Out>> Alt<'a, Out> for (F0,) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2, F3) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2, F3, F4) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.4)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2, F3, F4, F5) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.4)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.5)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2, F3, F4, F5, F6) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.4)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.5)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.6)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2, F3, F4, F5, F6, F7) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.4)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.5)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.6)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.7)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.4)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.5)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.6)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.7)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.8)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.4)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.5)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.6)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.7)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.8)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.9)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.4)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.5)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.6)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.7)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.8)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.9)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.10)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.4)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.5)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.6)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.7)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.8)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.9)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.10)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.11)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F12: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.4)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.5)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.6)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.7)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.8)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.9)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.10)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.11)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.12)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F12: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F13: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.4)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.5)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.6)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.7)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.8)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.9)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.10)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.11)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.12)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.13)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F12: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F13: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F14: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.4)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.5)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.6)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.7)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.8)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.9)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.10)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.11)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.12)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.13)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.14)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
impl<
    'a,
    Out,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F12: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F13: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F14: Fn(&Span<'a>) -> ParserResult<'a, Out>,
    F15: Fn(&Span<'a>) -> ParserResult<'a, Out>,
> Alt<'a, Out>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
        move |input: &Span<'a>| {
            let mut err = None;
            match (self.0)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.1)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.2)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.3)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.4)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.5)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.6)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.7)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.8)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.9)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.10)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.11)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.12)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.13)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.14)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            match (self.15)(input) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
            Err(err.unwrap())
        }
    }
}
pub trait Tuple<O> {
    fn tuple<'b>(&'b self) -> impl for<'a> Fn(&Span<'a>) -> ParserResult<'a, O> + 'b;
}
impl<F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>, O0> Tuple<(O0,)> for (F0,) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(&Span<'a>) -> ParserResult<'a, (O0,)> + 'b {
        for<'a> move |input: &'b Span<'a>| -> impl ParserResult<'a, (O0,)> + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            Ok((input, (F0,)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    O0,
    O1,
> Tuple<(O0, O1)> for (F0, F1) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(&Span<'a>) -> ParserResult<'a, (O0, O1)> + 'b {
        for<'a> move |input: &'b Span<'a>| -> impl ParserResult<'a, (O0, O1)> + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            Ok((input, (F0, F1)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    O0,
    O1,
    O2,
> Tuple<(O0, O1, O2)> for (F0, F1, F2) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(&Span<'a>) -> ParserResult<'a, (O0, O1, O2)> + 'b {
        for<'a> move |input: &'b Span<'a>| -> impl ParserResult<'a, (O0, O1, O2)> + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            Ok((input, (F0, F1, F2)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    O0,
    O1,
    O2,
    O3,
> Tuple<(O0, O1, O2, O3)> for (F0, F1, F2, F3) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(&Span<'a>) -> ParserResult<'a, (O0, O1, O2, O3)> + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<'a, (O0, O1, O2, O3)> + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            Ok((input, (F0, F1, F2, F3)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    F4: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O4>,
    O0,
    O1,
    O2,
    O3,
    O4,
> Tuple<(O0, O1, O2, O3, O4)> for (F0, F1, F2, F3, F4) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(&Span<'a>) -> ParserResult<'a, (O0, O1, O2, O3, O4)> + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<'a, (O0, O1, O2, O3, O4)> + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            let (rest, F4) = (self.4)(&input)?;
            input = rest;
            Ok((input, (F0, F1, F2, F3, F4)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    F4: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O4>,
    F5: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O5>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
> Tuple<(O0, O1, O2, O3, O4, O5)> for (F0, F1, F2, F3, F4, F5) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(&Span<'a>) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5)> + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<'a, (O0, O1, O2, O3, O4, O5)> + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            let (rest, F4) = (self.4)(&input)?;
            input = rest;
            let (rest, F5) = (self.5)(&input)?;
            input = rest;
            Ok((input, (F0, F1, F2, F3, F4, F5)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    F4: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O4>,
    F5: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O5>,
    F6: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O6>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
> Tuple<(O0, O1, O2, O3, O4, O5, O6)> for (F0, F1, F2, F3, F4, F5, F6) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(
        &Span<'a>,
    ) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6)> + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6)> + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            let (rest, F4) = (self.4)(&input)?;
            input = rest;
            let (rest, F5) = (self.5)(&input)?;
            input = rest;
            let (rest, F6) = (self.6)(&input)?;
            input = rest;
            Ok((input, (F0, F1, F2, F3, F4, F5, F6)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    F4: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O4>,
    F5: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O5>,
    F6: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O6>,
    F7: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O7>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
> Tuple<(O0, O1, O2, O3, O4, O5, O6, O7)> for (F0, F1, F2, F3, F4, F5, F6, F7) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(
        &Span<'a>,
    ) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7)> + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7)> + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            let (rest, F4) = (self.4)(&input)?;
            input = rest;
            let (rest, F5) = (self.5)(&input)?;
            input = rest;
            let (rest, F6) = (self.6)(&input)?;
            input = rest;
            let (rest, F7) = (self.7)(&input)?;
            input = rest;
            Ok((input, (F0, F1, F2, F3, F4, F5, F6, F7)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    F4: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O4>,
    F5: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O5>,
    F6: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O6>,
    F7: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O7>,
    F8: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O8>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
> Tuple<(O0, O1, O2, O3, O4, O5, O6, O7, O8)> for (F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(
        &Span<'a>,
    ) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8)> + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8)> + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            let (rest, F4) = (self.4)(&input)?;
            input = rest;
            let (rest, F5) = (self.5)(&input)?;
            input = rest;
            let (rest, F6) = (self.6)(&input)?;
            input = rest;
            let (rest, F7) = (self.7)(&input)?;
            input = rest;
            let (rest, F8) = (self.8)(&input)?;
            input = rest;
            Ok((input, (F0, F1, F2, F3, F4, F5, F6, F7, F8)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    F4: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O4>,
    F5: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O5>,
    F6: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O6>,
    F7: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O7>,
    F8: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O8>,
    F9: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O9>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
> Tuple<(O0, O1, O2, O3, O4, O5, O6, O7, O8, O9)>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(
        &Span<'a>,
    ) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9)> + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9)> + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            let (rest, F4) = (self.4)(&input)?;
            input = rest;
            let (rest, F5) = (self.5)(&input)?;
            input = rest;
            let (rest, F6) = (self.6)(&input)?;
            input = rest;
            let (rest, F7) = (self.7)(&input)?;
            input = rest;
            let (rest, F8) = (self.8)(&input)?;
            input = rest;
            let (rest, F9) = (self.9)(&input)?;
            input = rest;
            Ok((input, (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    F4: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O4>,
    F5: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O5>,
    F6: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O6>,
    F7: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O7>,
    F8: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O8>,
    F9: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O9>,
    F10: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O10>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
> Tuple<(O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10)>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(
        &Span<'a>,
    ) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10)> + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10)> + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            let (rest, F4) = (self.4)(&input)?;
            input = rest;
            let (rest, F5) = (self.5)(&input)?;
            input = rest;
            let (rest, F6) = (self.6)(&input)?;
            input = rest;
            let (rest, F7) = (self.7)(&input)?;
            input = rest;
            let (rest, F8) = (self.8)(&input)?;
            input = rest;
            let (rest, F9) = (self.9)(&input)?;
            input = rest;
            let (rest, F10) = (self.10)(&input)?;
            input = rest;
            Ok((input, (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    F4: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O4>,
    F5: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O5>,
    F6: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O6>,
    F7: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O7>,
    F8: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O8>,
    F9: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O9>,
    F10: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O10>,
    F11: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O11>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
> Tuple<(O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11)>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(
        &Span<'a>,
    ) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11)> + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<
            'a,
            (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11),
        > + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            let (rest, F4) = (self.4)(&input)?;
            input = rest;
            let (rest, F5) = (self.5)(&input)?;
            input = rest;
            let (rest, F6) = (self.6)(&input)?;
            input = rest;
            let (rest, F7) = (self.7)(&input)?;
            input = rest;
            let (rest, F8) = (self.8)(&input)?;
            input = rest;
            let (rest, F9) = (self.9)(&input)?;
            input = rest;
            let (rest, F10) = (self.10)(&input)?;
            input = rest;
            let (rest, F11) = (self.11)(&input)?;
            input = rest;
            Ok((input, (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    F4: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O4>,
    F5: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O5>,
    F6: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O6>,
    F7: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O7>,
    F8: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O8>,
    F9: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O9>,
    F10: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O10>,
    F11: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O11>,
    F12: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O12>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
    O12,
> Tuple<(O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12)>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(
        &Span<'a>,
    ) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12)> + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<
            'a,
            (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12),
        > + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            let (rest, F4) = (self.4)(&input)?;
            input = rest;
            let (rest, F5) = (self.5)(&input)?;
            input = rest;
            let (rest, F6) = (self.6)(&input)?;
            input = rest;
            let (rest, F7) = (self.7)(&input)?;
            input = rest;
            let (rest, F8) = (self.8)(&input)?;
            input = rest;
            let (rest, F9) = (self.9)(&input)?;
            input = rest;
            let (rest, F10) = (self.10)(&input)?;
            input = rest;
            let (rest, F11) = (self.11)(&input)?;
            input = rest;
            let (rest, F12) = (self.12)(&input)?;
            input = rest;
            Ok((input, (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    F4: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O4>,
    F5: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O5>,
    F6: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O6>,
    F7: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O7>,
    F8: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O8>,
    F9: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O9>,
    F10: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O10>,
    F11: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O11>,
    F12: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O12>,
    F13: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O13>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
    O12,
    O13,
> Tuple<(O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13)>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(
        &Span<'a>,
    ) -> ParserResult<
            'a,
            (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13),
        > + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<
            'a,
            (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13),
        > + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            let (rest, F4) = (self.4)(&input)?;
            input = rest;
            let (rest, F5) = (self.5)(&input)?;
            input = rest;
            let (rest, F6) = (self.6)(&input)?;
            input = rest;
            let (rest, F7) = (self.7)(&input)?;
            input = rest;
            let (rest, F8) = (self.8)(&input)?;
            input = rest;
            let (rest, F9) = (self.9)(&input)?;
            input = rest;
            let (rest, F10) = (self.10)(&input)?;
            input = rest;
            let (rest, F11) = (self.11)(&input)?;
            input = rest;
            let (rest, F12) = (self.12)(&input)?;
            input = rest;
            let (rest, F13) = (self.13)(&input)?;
            input = rest;
            Ok((input, (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13)))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    F4: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O4>,
    F5: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O5>,
    F6: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O6>,
    F7: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O7>,
    F8: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O8>,
    F9: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O9>,
    F10: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O10>,
    F11: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O11>,
    F12: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O12>,
    F13: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O13>,
    F14: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O14>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
    O12,
    O13,
    O14,
> Tuple<(O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14)>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(
        &Span<'a>,
    ) -> ParserResult<
            'a,
            (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14),
        > + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<
            'a,
            (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14),
        > + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            let (rest, F4) = (self.4)(&input)?;
            input = rest;
            let (rest, F5) = (self.5)(&input)?;
            input = rest;
            let (rest, F6) = (self.6)(&input)?;
            input = rest;
            let (rest, F7) = (self.7)(&input)?;
            input = rest;
            let (rest, F8) = (self.8)(&input)?;
            input = rest;
            let (rest, F9) = (self.9)(&input)?;
            input = rest;
            let (rest, F10) = (self.10)(&input)?;
            input = rest;
            let (rest, F11) = (self.11)(&input)?;
            input = rest;
            let (rest, F12) = (self.12)(&input)?;
            input = rest;
            let (rest, F13) = (self.13)(&input)?;
            input = rest;
            let (rest, F14) = (self.14)(&input)?;
            input = rest;
            Ok((
                input,
                (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14),
            ))
        }
    }
}
impl<
    F0: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O0>,
    F1: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O1>,
    F2: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O2>,
    F3: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O3>,
    F4: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O4>,
    F5: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O5>,
    F6: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O6>,
    F7: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O7>,
    F8: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O8>,
    F9: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O9>,
    F10: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O10>,
    F11: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O11>,
    F12: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O12>,
    F13: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O13>,
    F14: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O14>,
    F15: for<'a> Fn(&Span<'a>) -> ParserResult<'a, O15>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
    O12,
    O13,
    O14,
    O15,
> Tuple<(O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15)>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    fn tuple<'b>(
        &'b self,
    ) -> impl for<'a> Fn(
        &Span<'a>,
    ) -> ParserResult<
            'a,
            (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15),
        > + 'b {
        for<'a> move |
            input: &'b Span<'a>,
        | -> impl ParserResult<
            'a,
            (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15),
        > + 'b {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            let (rest, F1) = (self.1)(&input)?;
            input = rest;
            let (rest, F2) = (self.2)(&input)?;
            input = rest;
            let (rest, F3) = (self.3)(&input)?;
            input = rest;
            let (rest, F4) = (self.4)(&input)?;
            input = rest;
            let (rest, F5) = (self.5)(&input)?;
            input = rest;
            let (rest, F6) = (self.6)(&input)?;
            input = rest;
            let (rest, F7) = (self.7)(&input)?;
            input = rest;
            let (rest, F8) = (self.8)(&input)?;
            input = rest;
            let (rest, F9) = (self.9)(&input)?;
            input = rest;
            let (rest, F10) = (self.10)(&input)?;
            input = rest;
            let (rest, F11) = (self.11)(&input)?;
            input = rest;
            let (rest, F12) = (self.12)(&input)?;
            input = rest;
            let (rest, F13) = (self.13)(&input)?;
            input = rest;
            let (rest, F14) = (self.14)(&input)?;
            input = rest;
            let (rest, F15) = (self.15)(&input)?;
            input = rest;
            Ok((
                input,
                (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15),
            ))
        }
    }
}
pub fn token<'a>(
    token_kind: TokenKind,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, &'a LocatedToken<'a>> {
    move |input: &Span<'a>| {
        let location = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        if first.kind() == token_kind {
            Ok((input, first))
        } else {
            Err(
                ParserError::UnexpectedToken(
                        first.kind(),
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([token_kind]),
                        ),
                    )
                    .locate(location),
            )
        }
    }
}
pub trait MapParser<'a, O> {
    fn map<O2, F: Fn(O) -> O2>(
        self,
        wrapper: F,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, O2>;
    fn map_err<F: Fn(ParserError) -> ParserError>(
        self,
        wrapper: F,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, O>;
}
impl<'a, O, P: Fn(&Span<'a>) -> ParserResult<'a, O>> MapParser<'a, O> for P {
    fn map<O2, F: Fn(O) -> O2>(
        self,
        wrapper: F,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, O2> {
        move |span: &Span<'a>| {
            let (input, output) = self(span)?;
            Ok((input, wrapper(output)))
        }
    }
    fn map_err<F: Fn(ParserError) -> ParserError>(
        self,
        wrapper: F,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, O> {
        move |span: &Span<'a>| {
            let result = self(span);
            match result {
                Ok((input, output)) => Ok((input, output)),
                Err(e) => Err(e.map_error(&wrapper)),
            }
        }
    }
}
