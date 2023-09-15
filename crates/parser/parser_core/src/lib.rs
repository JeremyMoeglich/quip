#![feature(return_position_impl_trait_in_trait)]

use ast::Location;
use lexer::{Token, TokenKind};
mod error_union;
use logos::Logos;
use parser_proc::{generate_all_tuple_impls, generate_all_alt_impls};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Span<'a> {
    pub tokens: &'a [LocatedToken<'a>],
    pub start: Location,
}

#[derive(Debug, Clone)]
pub struct LocatedToken<'a> {
    pub start: Location,
    pub text: &'a str,
    pub token: Token<'a>,
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

#[derive(Error, Debug)]
pub enum TakeParserError {
    #[error("There weren't enough tokens left to take")]
    EndOfInput,
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
        self.tokens
            .last()
            .map(|t| t.column())
            .unwrap_or(self.start.column)
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
                .unwrap_or(self.start.line + chunk.iter().map(|t| t.new_lines()).sum::<usize>()),
            column: rest.first().map(|t| t.start.column).unwrap_or({
                if chunk.iter().map(|t| t.new_lines()).sum::<usize>() == 0 {
                    self.start.column + chunk.iter().map(|t| t.len()).sum::<usize>()
                } else {
                    chunk.last().unwrap().column()
                }
            }),
            index: rest
                .first()
                .map(|t| t.start.index)
                .unwrap_or(self.start.index + chunk.iter().map(|t| t.len()).sum::<usize>()),
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
        let chunk = chunk.try_into().unwrap(); // safe because chunk.len() == N due to take_n_token
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
        span.take_n_token(i).unwrap() // safe because i <= span.tokens.len()
    }
}

pub fn many0<'a, O, E: Clone>(
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O, E>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E> {
    move |input: &Span<'a>| {
        let mut input = input.clone();
        let mut output = vec![];
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
        let mut output = vec![];
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
        let mut output = vec![];

        loop {
            // Try to parse the main element first
            if let Ok((rest, o)) = parser(&input) {
                input = rest;
                output.push(o);
            } else {
                break;
            }

            // Now try to parse the separator
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
        let mut output = vec![];

        // Parse the first element, which must be present for separated_list1
        match parser(&input) {
            Ok((rest, o)) => {
                input = rest;
                output.push(o);
            }
            Err(e) => return Err(e), // Forward the underlying parser's error
        }

        loop {
            // Try to parse the separator
            if let Ok((rest, _)) = separator(&input) {
                input = rest;
            } else {
                break;
            }

            // Try to parse the main element
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

        // Parse the start delimiter
        let (rest, _) = start(&input)?;
        input = rest;

        // Parse the main content
        let (rest, o) = parser(&input)?;
        input = rest;

        // Parse the end delimiter
        let (rest, _) = end(&input)?;
        input = rest;

        Ok((input, o))
    }
}

#[derive(Error, Debug)]
pub enum TokenParserSubError {
    #[error("The token was not the expected kind")]
    WrongTokenKind,
}

combine_errors!(pub TokenParserError, TakeParserError, TokenParserSubError);

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

#[macro_export]
macro_rules! extract_token_data {
    ($located_token:expr, $token_kind:ident) => {
        match $located_token.token {
            Token::$token_kind(data) => Some(data),
            _ => None,
        }
    };
}

#[macro_export]
macro_rules! token_parser {
    (data $token_kind:ident) => {
        move |input: &Span<'_>| {
            let (input, first_span) = input.take_n_token(1)?;
            let first = &first_span.tokens[0];
            if first.kind() == lexer::TokenKind::$token_kind {
                match first.token {
                    Token::$token_kind(data) => Ok((input, data)),
                    _ => Err(parser_core::TokenParserSubError::WrongTokenKind.into()),
                }
            } else {
                Err(parser_core::TokenParserSubError::WrongTokenKind.into())
            }
        }
    };
    (nodata $token_kind:ident) => {
        move |input: &Span<'_>| {
            let (input, first_span) = input.take_n_token(1)?;
            let first = &first_span.tokens[0];
            if first.kind() == lexer::TokenKind::$token_kind {
                match first.token {
                    lexer::Token::$token_kind => Ok((input, ())),
                    _ => Err(parser_core::TokenParserSubError::WrongTokenKind.into()),
                }
            } else {
                Err(parser_core::TokenParserSubError::WrongTokenKind.into())
            }
        }
    };
}

pub trait Alt<'a, O, E> {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E>;
}

generate_all_alt_impls!(16);

pub trait Tuple<'a, O, E> {
    fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E>;
}

generate_all_tuple_impls!(16);


#[derive(Error, Debug)]
pub enum MoreThanOneError {
    #[error("The parser returned 0 elements, expected at least 1")]
    ZeroElements,
}

pub fn more_than_one<
    'a,
    O,
    E: From<MoreThanOneError>,
    P: Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E>,
>(
    parser: P,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E> {
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

#[cfg(test)]
mod tests {
    use crate::{create_span, tokenize};

    #[test]
    fn test_take_n() {
        let tokens = tokenize(r#"fn foo() { 1 + 2 }"#);
        let span = create_span(&tokens);
        let (first, rest) = span.take_n_token(3).unwrap();
        println!("{:?}", first);
        println!("{:?}", rest);
    }
}
