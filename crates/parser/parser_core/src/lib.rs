#![feature(return_position_impl_trait_in_trait)]
pub mod lexer;
pub use lexer::*;
use ast::Location;
mod error_union;
use logos::Logos;
use proc_macros::{generate_all_alt_impls, generate_all_tuple_impls};
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
        self.tokens
            .last()
            .map(|t| t.column())
            .unwrap_or(self.start.column)
    }
}

#[derive(Error, Debug, Clone)]
pub enum ParserError {
    #[error("Unexpected token {0:?}, expected one of {1:?}")]
    UnexpectedToken(TokenKind, Vec<TokenKind>), // Got, Expected
    #[error("Unexpected end of input")]
    EndOfInput,
    #[error("Parser inactive")]
    InactiveParser,
}
impl ParserError {
    pub fn locate(self, location: Location) -> LocatedParserError {
        LocatedParserError::new(self, location)
    }
}

#[derive(Error, Debug)]
#[error("{error} at {location}")]
pub struct LocatedParserError {
    pub error: ParserError,
    pub location: Location,
}

impl LocatedParserError {
    pub fn new(error: ParserError, location: Location) -> Self {
        LocatedParserError { error, location }
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

pub type ParserOutput<O> = Result<O, LocatedParserError>;
pub type ParserResult<'a, O> = Result<(Span<'a>, O), LocatedParserError>;

pub trait ParserResultTrait<'a, O> {
    fn to_output(self) -> ParserOutput<O>;
}

impl<'a, O> ParserResultTrait<'a, O> for ParserResult<'a, O> {
    fn to_output(self) -> ParserOutput<O> {
        self.map(|(_, o)| o)
    }
}

impl<'a> Span<'a> {
    pub fn new(code: &'a [LocatedToken], start: Location) -> Self {
        Self {
            tokens: code,
            start,
        }
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

    pub fn take_tokens<const N: usize>(&self) -> ParserResult<'a, &'a [LocatedToken<'a>; N]> {
        let (rest, chunk) = self.take_n_token(N)?;
        let chunk = chunk.tokens;
        let chunk = chunk.try_into().unwrap(); // safe because chunk.len() == N due to take_n_token
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
                Location {
                    index,
                    line,
                    column,
                }
            }
            None => Location {
                index: 0,
                line: 0,
                column: 0,
            },
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

pub fn many0<'a, O>(
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>> {
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

pub fn many1<'a, O>(
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>> {
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

pub fn separated_list0<'a, O, O_>(
    separator: impl Fn(&Span<'a>) -> ParserResult<'a, O_>,
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>> {
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
    separator: impl Fn(&Span<'a>) -> ParserResult<'a, ()>,
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>> {
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

pub fn delimited<'a, O, O1_, O2_>(
    start: impl Fn(&Span<'a>) -> ParserResult<'a, O1_>,
    parser: impl Fn(&Span<'a>) -> ParserResult<'a, O>,
    end: impl Fn(&Span<'a>) -> ParserResult<'a, O2_>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, O> {
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

pub fn preceded<'a, O1_, O>(
    first: impl Fn(&Span<'a>) -> ParserResult<'a, O1_>,
    second: impl Fn(&Span<'a>) -> ParserResult<'a, O>,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, O> {
    move |input: &Span<'a>| {
        let mut input = input.clone();

        // Run the first parser and discard its result
        let (rest, _) = first(&input)?;
        input = rest;

        // Run the second parser and keep its result
        let (rest, o) = second(&input)?;
        input = rest;

        Ok((input, o))
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
    fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O> + 'b;
}

generate_all_alt_impls!(16);

pub trait Tuple<'a, O> {
    fn tuple<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O> + 'b;
}

generate_all_tuple_impls!(16);

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
            Err(ParserError::UnexpectedToken(first.kind(), vec![token_kind]).locate(location))
        }
    }
}

pub trait MapParser<'a, O> {
    fn map<O2, F: Fn(O) -> O2>(self, wrapper: F) -> impl Fn(&Span<'a>) -> ParserResult<'a, O2>;
    fn map_err<F: Fn(ParserError) -> ParserError>(
        self,
        wrapper: F,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, O>;
}

impl<'a, O, P: Fn(&Span<'a>) -> ParserResult<'a, O>> MapParser<'a, O> for P {
    fn map<O2, F: Fn(O) -> O2>(self, wrapper: F) -> impl Fn(&Span<'a>) -> ParserResult<'a, O2> {
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
