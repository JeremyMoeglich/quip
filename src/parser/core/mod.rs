pub mod flatten;

use std::{collections::HashSet, fmt::Debug};

use derivative::Derivative;

use flatten::Flatable;

use crate::parser::lexer::{LocatedToken, TokenKind};

pub type TokenSlice<'a> = &'a [LocatedToken<'a>];

pub type RecoveryFunc<'a, T> = Box<dyn FnOnce() -> ParseSuccess<'a, T> + 'a>;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ParseErrorData<'a, T: Debug + Clone> {
    /// Information about the error and a way to recover from it
    pub error_token: Option<&'a LocatedToken<'a>>,
    pub valid_start: bool, // Whether the parser was able to parse tokens before the error
    #[derivative(Debug = "ignore")]
    pub recovery: RecoveryFunc<'a, T>, // generate a recovery solution,
    pub expected: HashSet<TokenKind>,
}

impl<'a, T: Debug + Clone> ParseErrorData<'a, T> {
    pub fn new(
        error_token: Option<&'a LocatedToken<'a>>,
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

fn chain<'a, T: Debug + Clone + 'a, U: Debug + Clone + 'a>(
    f1: impl SingleParser<'a, T> + 'a,
    f2: impl SingleParser<'a, U> + 'a,
) -> impl SingleParser<'a, (T, U)> + 'a {
    move |input| {
        let result1 = f1(input);
        match result1 {
            Ok((input, output1)) => match f2.parse(input) {
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
                    match f2.parse(input) {
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
    }
}

pub trait SingleParser<'a, T: Debug + Clone + 'a> {
    /// use the parser
    fn final_parse(self, input: ParserInput<'a>) -> ParseResult<'a, T>;
    fn final_force(self, input: ParserInput<'a>) -> ParseSuccess<'a, T> {
        let result = self.final_parse(input);
        match result {
            Ok((input, output)) => (input, output),
            Err(err) => (err.recovery)(),
        }
    }
    fn map_result<U: Debug + Clone + 'a>(
        self,
        f: impl Fn(T) -> U + 'a,
    ) -> Box<dyn FnOnce(ParserInput<'a>) -> ParseResult<'a, U> + 'a> {
        Box::new(move |input| {
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
        })
    }
    /// Take two parsers and combine them into one
    fn chain<U: Debug + Clone + 'a>(
        &self,
        f: impl Parser<'a, U> + 'a,
    ) -> Box<dyn Fn(ParserInput<'a>) -> ParseResult<'a, (T, U)> + 'a> {
        chain(self, f)
    }

    fn alt(
        self,
        f: impl Parser<'a, T> + 'a,
    ) -> Box<dyn Fn(ParserInput<'a>) -> ParseResult<'a, T> + 'a> {
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

    fn branch<U: Debug + Clone + 'a>(
        self,
        f1: impl Parser<'a, U> + 'a,
        f2: impl Parser<'a, U> + 'a,
    ) -> Box<dyn Fn(ParserInput<'a>) -> ParseResult<'a, (T, U)> + 'a> {
        let second_parser = |self_output: T| {
            let output_ref = &self_output;
            move |input| match f1.parse(input) {
                Ok((input, output1)) => Ok((input, (output_ref.clone(), output1))),
                Err(err1) => match f2.parse(input) {
                    Ok((input, output2)) => Ok((input, (output_ref.clone(), output2))),
                    Err(err2) => Err(ParseErrorData::new(
                        match err1.valid_start {
                            true => err1.error_token,
                            false => err2.error_token,
                        },
                        err1.valid_start || err2.valid_start,
                        match err1.valid_start {
                            true => Box::new(move || {
                                let (input, output1) = (err1.recovery)();
                                (input, (output_ref.clone(), output1))
                            }),
                            false => Box::new(move || {
                                let (input, output2) = (err2.recovery)();
                                (input, (output_ref.clone(), output2))
                            }),
                        },
                        err1.expected.union(&err2.expected).cloned().collect(),
                    )),
                },
            }
        };
        Box::new(move |input| {
            let self_result = self.parse(input);
            match self_result {
                Ok((input, self_output)) => second_parser(self_output)(input),
                Err(self_error) => Err(not_it(
                    self_error.error_token,
                    self_error.valid_start,
                    Box::new(move || {
                        let (input, self_output) = (self_error.recovery)();
                        second_parser(self_output).force(input)
                    }),
                    self_error.expected,
                )),
            }
        })
    }

    /// flatten nested tuples to a single tuple ((p1, p2), p3) -> ((r1, r2), r3) -> (r1, r2, r3)
    fn flattened<O: Clone + Debug>(self) -> Box<dyn Fn(ParserInput<'a>) -> ParseResult<'a, O> + 'a>
    where
        T: Flatable<Flattened = O>,
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

pub trait Parser<'a, T: Debug + Clone + 'a>: SingleParser<'a, T> {
    fn parse(self, input: ParserInput<'a>) -> ParseResult<'a, T>;
    fn force(&self, input: ParserInput<'a>) -> ParseSuccess<'a, T> {
        let result = self.parse(input);
        match result {
            Ok((input, output)) => (input, output),
            Err(err) => (err.recovery)(),
        }
    }
}

impl<'a, P, T: Debug + Clone + 'a> SingleParser<'a, T> for P
where
    P: FnOnce(TokenSlice<'a>) -> ParseResult<'a, T>,
{
    fn final_parse(self, input: ParserInput<'a>) -> ParseResult<'a, T> {
        self(input)
    }
}

pub fn not_it<'a, T: Debug + Clone>(
    error_token: Option<&'a LocatedToken<'a>>,
    valid_start: bool,
    recovery: RecoveryFunc<'a, T>,
    expected: HashSet<TokenKind>,
) -> ParseErrorData<'a, T> {
    ParseErrorData::new(error_token, valid_start, recovery, expected)
}
