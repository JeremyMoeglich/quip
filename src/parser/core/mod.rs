pub mod flatten;
pub mod chaining;

use std::{collections::HashSet, fmt::Debug};

use derivative::Derivative;

use flatten::Flatable;

use crate::parser::lexer::{LocatedToken, TokenKind};

use super::common::map_recovery;

pub type TokenSlice<'a> = &'a [LocatedToken];

pub type RecoveryFunc<'a, T> = Box<dyn FnOnce() -> ParseSuccess<'a, T> + 'a>;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ParseErrorData<'a, T: Debug + Clone> {
    /// Information about the error and a way to recover from it
    pub error_token: Option<&'a LocatedToken>,
    pub valid_start: bool, // Whether the parser was able to parse tokens before the error
    #[derivative(Debug = "ignore")]
    pub recovery: RecoveryFunc<'a, T>, // generate a recovery solution,
    pub expected: HashSet<TokenKind>,
}

impl<'a, T: Debug + Clone> ParseErrorData<'a, T> {
    pub fn new(
        error_token: Option<&'a LocatedToken>,
        valid_start: bool,
        recovery: RecoveryFunc<'a, T>,
        expected: HashSet<TokenKind>,
    ) -> Self {
        ParseErrorData {
            error_token,
            valid_start,
            recovery,
            expected,
        }
    }
}

pub type ParseSuccess<'a, T: Debug + Clone> = (TokenSlice<'a>, T);

/// The type returned by a parser
pub type ParseResult<'a, T: Debug + Clone> = Result<ParseSuccess<'a, T>, ParseErrorData<'a, T>>;

/// The single argument of a parser
pub type ParserInput<'a> = TokenSlice<'a>;

pub trait Parser: Sized {
    type Output: Debug + Clone + 'static;
    /// use the parser
    fn parse<'a>(&self, input: ParserInput<'a>) -> ParseResult<'a, Self::Output>;
    fn force<'a>(&self, input: ParserInput<'a>) -> ParseSuccess<'a, Self::Output> {
        let result = self.parse(input);
        match result {
            Ok((input, output)) => (input, output),
            Err(err) => (err.recovery)(),
        }
    }
    fn map_result<U: Debug + Clone, F: Fn(Self::Output) -> U>(
        self,
        f: &'static F,
    ) -> Box<dyn for<'a> Fn(ParserInput<'a>) -> ParseResult<'a, U>>
    where
        Self: 'static,
    {
        Box::new(
            for<'a> move |input: ParserInput<'a>| -> ParseResult<'a, U> {
                let result = self.parse(input);
                match result {
                    Ok((input, output)) => Ok((input, f(output))),
                    Err(err) => Err(not_it(
                        err.error_token,
                        err.valid_start,
                        Box::new(move || {
                            let (input, output) = (err.recovery)();
                            (input, f(output))
                        }),
                        err.expected,
                    )),
                }
            },
        )
    }
    /// Take two parsers and combine them into one
    fn chain<U: Debug + Clone + 'static>(
        &'static self,
        f: &'static impl Parser<Output = U>,
    ) -> Box<dyn for<'a> Fn(ParserInput<'a>) -> ParseResult<'a, (Self::Output, U)>>
    where
        Self: 'static,
    {
        Box::new(move |input| {
            let result1 = self.parse(input);
            match result1 {
                Ok((input, output1)) => match f.parse(input) {
                    Ok((input, output2)) => Ok((input, (output1, output2))),
                    Err(err2) => Err(ParseErrorData::new(
                        err2.error_token,
                        true,
                        Box::new(move || {
                            let (input, output2) = (err2.recovery)();
                            (input, (output1, output2))
                        }),
                        err2.expected,
                    )),
                },
                Err(err1) => Err(ParseErrorData::new(
                    err1.error_token,
                    err1.valid_start,
                    Box::new(move || {
                        let (input, output1) = (err1.recovery)();
                        match f.parse(input) {
                            Ok((input, output2)) => (input, (output1, output2)),
                            Err(err2) => {
                                let (input, output2) = (err2.recovery)();
                                (input, (output1, output2))
                            }
                        }
                    }),
                    err1.expected,
                )),
            }
        })
    }

    fn alt(
        self,
        f: impl Parser<Output = Self::Output> + 'static,
    ) -> Box<dyn for<'a> Fn(ParserInput<'a>) -> ParseResult<'a, Self::Output>>
    where
        Self: 'static,
    {
        Box::new(move |input| {
            let result1 = self.parse(input);
            match result1 {
                Ok((input, output1)) => Ok((input, output1)),
                Err(err1) => match f.parse(input) {
                    Ok((input, output2)) => Ok((input, output2)),
                    Err(err2) => Err(ParseErrorData::new(
                        match err1.valid_start {
                            true => err1.error_token,
                            false => err2.error_token,
                        },
                        err1.valid_start || err2.valid_start,
                        match err1.valid_start {
                            true => Box::new(move || (err1.recovery)()),
                            false => Box::new(move || (err2.recovery)()),
                        },
                        err1.expected.union(&err2.expected).cloned().collect(),
                    )),
                },
            }
        })
    }

    fn branch<U: Debug + Clone + 'static>(
        self,
        f1: &'static impl Parser<Output = U>,
        f2: &'static impl Parser<Output = U>,
    ) -> Box<dyn for<'a> Fn(ParserInput<'a>) -> ParseResult<'a, (Self::Output, U)>>
    where
        Self: 'static,
    {
        Box::new(move |input| match self.parse(input) {
            Ok((input, self_output)) => match f1.parse(input) {
                Ok((input, output)) => Ok((input, (self_output, output))),
                Err(err1) => match f2.parse(input) {
                    Ok((input, output)) => Ok((input, (self_output, output))),
                    Err(err2) => Err(ParseErrorData::new(
                        match err1.valid_start {
                            true => err1.error_token,
                            false => err2.error_token,
                        },
                        err1.valid_start || err2.valid_start,
                        match err1.valid_start {
                            true => map_recovery(err1.recovery, move |output| {
                                (self_output.clone(), output)
                            }),
                            false => map_recovery(err2.recovery, move |output| {
                                (self_output.clone(), output)
                            }),
                        },
                        err1.expected.union(&err2.expected).cloned().collect(),
                    )),
                },
            },
            Err(self_error) => Err(not_it(
                self_error.error_token,
                self_error.valid_start,
                Box::new(move || {
                    let (input, self_output) = (self_error.recovery)();
                    match f1.parse(input) {
                        Ok((input, output)) => (input, (self_output, output)),
                        Err(err1) => match f2.parse(input) {
                            Ok((input, output)) => (input, (self_output, output)),
                            Err(err2) => {
                                let (input, output) = match err1.valid_start {
                                    true => (err1.recovery)(),
                                    false => (err2.recovery)(),
                                };
                                (input, (self_output, output))
                            }
                        },
                    }
                }),
                self_error.expected,
            )),
        })
    }

    /// flatten nested tuples to a single tuple ((p1, p2), p3) -> ((r1, r2), r3) -> (r1, r2, r3)
    fn flattened<O: Clone + Debug>(
        self,
    ) -> Box<dyn for<'a> Fn(ParserInput<'a>) -> ParseResult<'a, O>>
    where
        Self::Output: Flatable<Flattened = O>,
        Self: 'static,
    {
        Box::new(move |input| {
            let result = self.parse(input);
            match result {
                Ok((input, output)) => Ok((input, output.flatten())),
                Err(err) => Err(ParseErrorData::new(
                    err.error_token,
                    err.valid_start,
                    Box::new(move || {
                        let (input, output) = (err.recovery)();
                        (input, output.flatten())
                    }),
                    err.expected,
                )),
            }
        })
    }
}

impl<T: Debug + Clone + 'static, P> Parser for P
where
    P: for<'b> Fn(TokenSlice<'b>) -> ParseResult<'b, T>,
{
    type Output = T;
    fn parse<'a>(&self, input: ParserInput<'a>) -> ParseResult<'a, T> {
        self(input)
    }
}

pub fn not_it<'a, T: Debug + Clone>(
    error_token: Option<&'a LocatedToken>,
    valid_start: bool,
    recovery: RecoveryFunc<'a, T>,
    expected: HashSet<TokenKind>,
) -> ParseErrorData<'a, T> {
    ParseErrorData::new(error_token, valid_start, recovery, expected)
}
