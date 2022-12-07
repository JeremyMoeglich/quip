use std::fmt::Debug;

use derivative::Derivative;

use super::flatten::Flatable;

use crate::parser::lexer::{LocatedToken, TokenKind};

pub type TokenSlice<'a> = &'a [LocatedToken<'a>];

pub type RecoveryResult<'a, T> = (TokenSlice<'a>, T);

pub type RecoveryFunc<'a, T> = Box<dyn FnOnce() -> RecoveryResult<'a, T> + 'a>;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ParseErrorData<'a, T: Debug + Clone> {
    /// Information about the error and a way to recover from it
    pub error_token: Option<&'a LocatedToken<'a>>,
    pub valid_start: bool, // Whether the parser was able to parse tokens before the error
    #[derivative(Debug = "ignore")]
    pub recovery: RecoveryFunc<'a, T>, // generate a recovery solution,
    pub expected: Vec<TokenKind>,
}

impl<'a, T: Debug + Clone> ParseErrorData<'a, T> {
    pub fn new(
        error_token: Option<&'a LocatedToken<'a>>,
        valid_start: bool,
        recovery: RecoveryFunc<'a, T>,
        expected: Vec<TokenKind>,
    ) -> Self {
        ParseErrorData {
            error_token,
            valid_start,
            recovery,
            expected,
        }
    }
}

pub type ParseResult<'a, T: Debug + Clone> = Result<(TokenSlice<'a>, T), ParseErrorData<'a, T>>;
/// The type returned by a parser
pub type ParserInput<'a> = TokenSlice<'a>;
/// The single argument of a parser

pub trait Parser<'a, T: Debug + Clone> {
    /// use the parser
    fn parse(&mut self, input: ParserInput<'a>) -> ParseResult<'a, T>;

    /// Take two parsers and put their result in a tuple
    /// if an error occurs, create a recovery function that will try to recover from errors in either parser
    fn chain<U: Debug + Clone>(
        &mut self,
        f: &'a (impl Parser<'a, U> + 'a),
    ) -> Box<dyn FnMut(ParserInput<'a>) -> ParseResult<'a, (T, U)> + 'a> {
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

    /// flatten nested tuples to a single tuple ((p1, p2), p3) -> ((r1, r2), r3) -> (r1, r2, r3)
    fn flattened<O: Clone + Debug>(
        &self,
    ) -> Box<dyn FnMut(ParserInput<'a>) -> ParseResult<'a, O> + 'a>
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

impl<'a, P, T: Debug + Clone> Parser<'a, T> for P
where
    P: FnMut(TokenSlice<'a>) -> ParseResult<'a, T>,
{
    fn parse(&mut self, input: ParserInput<'a>) -> ParseResult<'a, T> {
        self(input)
    }
}
