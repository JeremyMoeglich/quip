use crate::{
    ast::Location,
    combine_errors,
    lexer::{Token, TokenKind},
};
mod error_union;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Span<'a> {
    pub tokens: &'a [LocatedToken<'a>],
    pub start: Location,
}

#[derive(Debug)]
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

        let last_token = chunk.last().unwrap(); // safe because n > 0
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
        Ok((rest, chunk[0]))
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

pub fn separated_list0<'a, O, E: Clone>(
    separator: impl Fn(&Span<'a>) -> ParserResult<'a, (), E>,
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

pub fn delimited<'a, O, E: From<E1> + From<E2> + From<E3>, E1, E2, E3>(
    start: impl Fn(&Span<'a>) -> ParserResult<'a, (), E1>,
    end: impl Fn(&Span<'a>) -> ParserResult<'a, (), E2>,
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O, E3>,
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
            if first.kind() == TokenKind::$token_kind {
                match first.token {
                    Token::$token_kind(data) => Ok((input, data)),
                    _ => Err(crate::core::TokenParserSubError::WrongTokenKind.into()),
                }
            } else {
                Err(crate::core::TokenParserSubError::WrongTokenKind.into())
            }
        }
    };
    (nodata $token_kind:ident) => {
        move |input: &Span<'_>| {
            let (input, first_span) = input.take_n_token(1)?;
            let first = &first_span.tokens[0];
            if first.kind() == TokenKind::$token_kind {
                match first.token {
                    crate::lexer::Token::$token_kind => Ok((input, ())),
                    _ => Err(crate::core::TokenParserSubError::WrongTokenKind.into()),
                }
            } else {
                Err(crate::core::TokenParserSubError::WrongTokenKind.into())
            }
        }
    };
}

pub trait Alt<'a, O, E> {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E>;
}

impl<'a, O, E, P: Fn(&Span) -> ParserResult<'a, O, E>> Alt<'a, O, E> for (P,) {
    fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E> {
        move |input: &Span<'a>| (self.0)(input)
    }
}

macro_rules! tuple_index {
    ($tuple:expr, $idx:tt) => {
        $tuple.$idx
    };
}

macro_rules! impl_alt {
    ($($name:ident, $idx:tt),+) => {
        impl<'a, Out, Err, $($name: Fn(&Span) -> ParserResult<'a, Out, Err>),+> Alt<'a, Out, Err> for ($($name,)+) {
            fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
                move |input: &Span<'a>| {
                    let mut err = None;
                    $(
                        match tuple_index!(self, $idx)(input) {
                            Ok(res) => return Ok(res),
                            Err(e) => {
                                if err.is_none() {
                                    err = Some(e);
                                }
                            }
                        }
                    )*
                    Err(err.unwrap()) // We're guaranteed to have an error if we've made it this far
                }
            }
        }
    };
}

macro_rules! impl_alts {
    ($($name:ident, $idx:tt),+) => {
        impl_alt!($($name, $idx),+);
    };
}

impl_alts!(
    A, 0, B, 1, C, 2, D, 3, E, 4, F, 5, G, 6, H, 7, I, 8, J, 9, K, 10, L, 11, M, 12, N, 13, O, 14,
    P, 15
);

pub trait Tuple<'a, O, E> {
    fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E>;
}

impl<'a, O, E, P: Fn(&Span<'a>) -> ParserResult<'a, O, E>> Tuple<'a, (O,), E> for (P,) {
    fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O,), E> {
        move |input: &Span<'a>| {
            let (rest, o) = (self.0)(input)?;
            Ok((rest, (o,)))
        }
    }
}

macro_rules! impl_tuple {
    ($($name:ident : $O:ident, $idx:tt),+) => {
        impl<'a, Err, $($name: Fn(&Span<'a>) -> ParserResult<'a, $O, Err>),+, $($O,)+> Tuple<'a, ($($O,)+), Err> for ($($name,)+) {
            fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, ($($O,)+), Err> {
                move |input: &Span<'a>| {
                    let mut input = input.clone();
                    $(
                        let (rest, $name) = tuple_index!(self, $idx)(&input)?;
                        input = rest;
                    )*
                    Ok((input, ($($name,)+)))
                }
            }
        }
    };
}

macro_rules! impl_tuples {
    ($($name:ident : $O:ident, $idx:tt),+) => {
        impl_tuple!($($name : $O, $idx),+);
    };
}

impl_tuples!(
    A: O1,
    0,
    B: O2,
    1,
    C: O3,
    2,
    D: O4,
    3,
    E: O5,
    4,
    F: O6,
    5,
    G: O7,
    6,
    H: O8,
    7,
    I: O9,
    8,
    J: O10,
    9,
    K: O11,
    10,
    L: O12,
    11,
    M: O13,
    12,
    N: O14,
    13,
    O: O15,
    14,
    P: O16,
    15
);

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
    use crate::lexer::{create_span, tokenize};

    #[test]
    fn test_take_n() {
        let tokens = tokenize(r#"fn foo() { 1 + 2 }"#);
        let span = create_span(&tokens);
        let (first, rest) = span.take_n_token(3).unwrap();
        println!("{:?}", first);
        println!("{:?}", rest);
    }
}
