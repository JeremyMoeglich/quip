#![feature(prelude_import)]
#![feature(return_position_impl_trait_in_trait)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use ast::Location;
use lexer::{Token, TokenKind};
mod error_union {}
use logos::Logos;
use parser_proc::{generate_all_tuple_impls, generate_all_alt_impls};
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
    Span {
        start: Location {
            column: 0,
            line: 0,
            index: 0,
        },
        tokens,
    }
}
pub enum TakeParserError {
    #[error("There weren't enough tokens left to take")]
    EndOfInput,
}
#[allow(unused_qualifications)]
impl std::error::Error for TakeParserError {}
#[allow(unused_qualifications)]
impl std::fmt::Display for TakeParserError {
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            TakeParserError::EndOfInput {} => {
                __formatter
                    .write_fmt(format_args!("There weren\'t enough tokens left to take"))
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TakeParserError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "EndOfInput")
    }
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
pub type ParserResult<'a, O, E> = Result<(Span<'a>, O), E>;
impl<'a> Span<'a> {
    pub fn new(code: &'a [LocatedToken], location: Location) -> Self {
        Self {
            tokens: code,
            start: location,
        }
    }
    pub fn take_n_token(&self, n: usize) -> ParserResult<'a, Span<'a>, TakeParserError> {
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
            return Err(TakeParserError::EndOfInput);
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
    ) -> ParserResult<'a, &'a [LocatedToken<'a>; N], TakeParserError> {
        let (rest, chunk) = self.take_n_token(N)?;
        let chunk = chunk.tokens;
        let chunk = chunk.try_into().unwrap();
        Ok((rest, chunk))
    }
    pub fn take_token(&self) -> ParserResult<'a, LocatedToken<'a>, TakeParserError> {
        let (rest, chunk) = self.take_tokens::<1>()?;
        Ok((rest, chunk[0].clone()))
    }
    pub fn kind(&self) -> Result<TokenKind, TakeParserError> {
        match self.tokens.first() {
            Some(token) => Ok(token.kind()),
            None => Err(TakeParserError::EndOfInput),
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
pub fn many0<'a, O, E: Clone>(
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O, E>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E> {
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
pub fn many1<'a, O, E: Clone>(
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O, E>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E> {
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
pub fn separated_list0<'a, O, O_, E>(
    separator: impl Fn(&Span<'a>) -> ParserResult<'a, O_, E>,
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O, E>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E> {
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
    separator: impl Fn(&Span<'a>) -> ParserResult<'a, (), E>,
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O, E>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E> {
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
pub fn delimited<'a, O, O1_, O2_, E: From<E1> + From<E2> + From<E3>, E1, E2, E3>(
    start: impl Fn(&Span<'a>) -> ParserResult<'a, O1_, E1>,
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O, E3>,
    end: impl Fn(&Span<'a>) -> ParserResult<'a, O2_, E2>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E> {
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
pub enum TokenParserSubError {
    #[error("The token was not the expected kind")]
    WrongTokenKind,
}
#[allow(unused_qualifications)]
impl std::error::Error for TokenParserSubError {}
#[allow(unused_qualifications)]
impl std::fmt::Display for TokenParserSubError {
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            TokenParserSubError::WrongTokenKind {} => {
                __formatter
                    .write_fmt(format_args!("The token was not the expected kind"))
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TokenParserSubError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "WrongTokenKind")
    }
}
use std::error::Error;
use std::fmt::{self, Display, Formatter};
pub enum TokenParserError {
    TakeParserError(TakeParserError),
    TokenParserSubError(TokenParserSubError),
}
#[automatically_derived]
impl ::core::fmt::Debug for TokenParserError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            TokenParserError::TakeParserError(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "TakeParserError",
                    &__self_0,
                )
            }
            TokenParserError::TokenParserSubError(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "TokenParserSubError",
                    &__self_0,
                )
            }
        }
    }
}
impl From<TakeParserError> for TokenParserError {
    fn from(err: TakeParserError) -> Self {
        TokenParserError::TakeParserError(err)
    }
}
impl From<TokenParserSubError> for TokenParserError {
    fn from(err: TokenParserSubError) -> Self {
        TokenParserError::TokenParserSubError(err)
    }
}
impl Display for TokenParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenParserError::TakeParserError(err) => {
                f.write_fmt(format_args!("{0}", err))
            }
            TokenParserError::TokenParserSubError(err) => {
                f.write_fmt(format_args!("{0}", err))
            }
        }
    }
}
impl Error for TokenParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TokenParserError::TakeParserError(err) => Some(err),
            TokenParserError::TokenParserSubError(err) => Some(err),
        }
    }
}
pub fn token<'a>(
    token_kind: TokenKind,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, &'a LocatedToken<'a>, TokenParserError> {
    move |input: &Span<'a>| {
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        if first.kind() == token_kind {
            Ok((input, first))
        } else {
            Err(TokenParserSubError::WrongTokenKind.into())
        }
    }
}
pub trait Alt<'a, O, E> {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E>;
}
impl<'a, Out, Err, F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>> Alt<'a, Out, Err>
for (F0,) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1, F2) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1, F2, F3) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1, F2, F3, F4) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1, F2, F3, F4, F5) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1, F2, F3, F4, F5, F6) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1, F2, F3, F4, F5, F6, F7) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F12: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F12: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F13: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F12: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F13: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F14: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F12: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F13: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F14: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
    F15: Fn(&Span<'a>) -> ParserResult<'a, Out, Err>,
> Alt<'a, Out, Err>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
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
pub trait Tuple<'a, O, E> {
    fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E>;
}
impl<'a, Err, F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>, O0> Tuple<'a, (O0,), Err>
for (F0,) {
    fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O0,), Err> {
        move |input: &Span<'a>| {
            let mut input = input.clone();
            let (rest, F0) = (self.0)(&input)?;
            input = rest;
            Ok((input, (F0,)))
        }
    }
}
impl<
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    O0,
    O1,
> Tuple<'a, (O0, O1), Err> for (F0, F1) {
    fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O0, O1), Err> {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    O0,
    O1,
    O2,
> Tuple<'a, (O0, O1, O2), Err> for (F0, F1, F2) {
    fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O0, O1, O2), Err> {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    O0,
    O1,
    O2,
    O3,
> Tuple<'a, (O0, O1, O2, O3), Err> for (F0, F1, F2, F3) {
    fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O0, O1, O2, O3), Err> {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
    O0,
    O1,
    O2,
    O3,
    O4,
> Tuple<'a, (O0, O1, O2, O3, O4), Err> for (F0, F1, F2, F3, F4) {
    fn tuple(
        &'a self,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O0, O1, O2, O3, O4), Err> {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
> Tuple<'a, (O0, O1, O2, O3, O4, O5), Err> for (F0, F1, F2, F3, F4, F5) {
    fn tuple(
        &'a self,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5), Err> {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
> Tuple<'a, (O0, O1, O2, O3, O4, O5, O6), Err> for (F0, F1, F2, F3, F4, F5, F6) {
    fn tuple(
        &'a self,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6), Err> {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
> Tuple<'a, (O0, O1, O2, O3, O4, O5, O6, O7), Err> for (F0, F1, F2, F3, F4, F5, F6, F7) {
    fn tuple(
        &'a self,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7), Err> {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
> Tuple<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8), Err>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    fn tuple(
        &'a self,
    ) -> impl Fn(
        &Span<'a>,
    ) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8), Err> {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
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
> Tuple<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9), Err>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    fn tuple(
        &'a self,
    ) -> impl Fn(
        &Span<'a>,
    ) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9), Err> {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
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
> Tuple<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10), Err>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    fn tuple(
        &'a self,
    ) -> impl Fn(
        &Span<'a>,
    ) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10), Err> {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
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
> Tuple<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11), Err>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    fn tuple(
        &'a self,
    ) -> impl Fn(
        &Span<'a>,
    ) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11), Err> {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
    F12: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
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
> Tuple<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12), Err>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    fn tuple(
        &'a self,
    ) -> impl Fn(
        &Span<'a>,
    ) -> ParserResult<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12), Err> {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
    F12: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
    F13: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
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
> Tuple<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13), Err>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    fn tuple(
        &'a self,
    ) -> impl Fn(
        &Span<'a>,
    ) -> ParserResult<
            'a,
            (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13),
            Err,
        > {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
    F12: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
    F13: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
    F14: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
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
> Tuple<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14), Err>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    fn tuple(
        &'a self,
    ) -> impl Fn(
        &Span<'a>,
    ) -> ParserResult<
            'a,
            (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14),
            Err,
        > {
        move |input: &Span<'a>| {
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
    'a,
    Err,
    F0: Fn(&Span<'a>) -> ParserResult<'a, O0, Err>,
    F1: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
    F2: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
    F3: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
    F4: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
    F5: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
    F6: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
    F7: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
    F8: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
    F9: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
    F10: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
    F11: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
    F12: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
    F13: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
    F14: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
    F15: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
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
> Tuple<'a, (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15), Err>
for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    fn tuple(
        &'a self,
    ) -> impl Fn(
        &Span<'a>,
    ) -> ParserResult<
            'a,
            (O0, O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15),
            Err,
        > {
        move |input: &Span<'a>| {
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
pub enum MoreThanOneError {
    #[error("The parser returned 0 elements, expected at least 1")]
    ZeroElements,
}
#[allow(unused_qualifications)]
impl std::error::Error for MoreThanOneError {}
#[allow(unused_qualifications)]
impl std::fmt::Display for MoreThanOneError {
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            MoreThanOneError::ZeroElements {} => {
                __formatter
                    .write_fmt(
                        format_args!(
                            "The parser returned 0 elements, expected at least 1"
                        ),
                    )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MoreThanOneError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "ZeroElements")
    }
}
pub fn more_than_one<
    'a,
    O,
    E: From<MoreThanOneError>,
    P: Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E>,
>(parser: P) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E> {
    move |span: &Span<'a>| {
        let (input, output) = parser(span)?;
        if output.len() != 1 {
            return Err(MoreThanOneError::ZeroElements.into());
        }
        Ok((input, output))
    }
}
pub fn map<'a, O, O2, E, P: Fn(&Span<'a>) -> ParserResult<'a, O, E>>(
    parser: P,
    wrapper: impl Fn(O) -> O2,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, O2, E> {
    move |span: &Span<'a>| {
        let (input, output) = parser(span)?;
        Ok((input, wrapper(output)))
    }
}
