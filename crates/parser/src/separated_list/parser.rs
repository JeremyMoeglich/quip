use parser_core::*;

use super::structure::{StrictSeparatedList, TrailingSeparatedList};

/// Parses a separated list of elements with optional termination, returning a `TrailingSeparatedList`.
///
/// # Returns
/// A function that takes a `Span<'a>` and returns a `ParserResult<'a, (TrailingSeparatedList<PO, SO>, TO)>`.
/// The `TrailingSeparatedList` contains the parsed elements and separators, and the second item is the result of the termination parser.
#[inline]
pub fn trailing_separated_list<
    'a,
    PO,
    PR: IntoParserResult<'a, PO>,
    SO,
    SR: IntoParserResult<'a, SO>,
    TO,
    TR: IntoParserResult<'a, TO>,
>(
    separator: impl Fn(Span<'a>) -> SR,
    parser: impl Fn(Span<'a>) -> PR,
    termination_parser: impl Fn(Span<'a>) -> TR,
    consume_termination_tokens: bool,
    require_one: bool,
) -> impl Fn(Span<'a>) -> ParserResult<'a, (TrailingSeparatedList<PO, SO>, TO)> {
    move |mut input: Span<'a>| {
        let mut list = TrailingSeparatedList::new();

        if require_one {
            match parser(input).into_parser_result() {
                Ok((rest, o)) => {
                    input = rest;
                    list.push_value(o).expect("Failed to push value in TrailingSeparatedList");
                }
                Err(e) => return Err(e),
            }
        };

        loop {
            // Try to parse the separator
            match separator(input).into_parser_result() {
                Ok((rest, sep)) => {
                    input = rest;
                    // Safe to unwrap as the parser enforces correct alternation
                    list.push_separator(sep).expect("Failed to push separator in TrailingSeparatedList");
                }
                Err(_) => {
                    // Attempt to parse the termination
                    match termination_parser(input).into_parser_result() {
                        Ok((rest, o)) => {
                            if consume_termination_tokens {
                                return Ok((rest, (list, o)));
                            } else {
                                return Ok((input, (list, o)));
                            }
                        }
                        Err(e) => {
                            // Termination parsing failed; return the error
                            return Err(e);
                        }
                    }
                }
            }

            // Try to parse the next value
            match parser(input).into_parser_result() {
                Ok((rest, o)) => {
                    input = rest;
                    // Safe to unwrap as the parser enforces correct alternation
                    list.push_value(o).expect("Failed to push value in TrailingSeparatedList");
                }
                Err(_) => {
                    // Attempt to parse the termination after a separator
                    match termination_parser(input).into_parser_result() {
                        Ok((rest, o)) => {
                            if consume_termination_tokens {
                                return Ok((rest, (list, o)));
                            } else {
                                return Ok((input, (list, o)));
                            }
                        }
                        Err(e) => {
                            // Termination parsing failed; return the error
                            return Err(e);
                        }
                    }
                }
            }
        }
    }
}

/// Parses a separated list of elements with optional termination, returning a `StrictSeparatedList`.
///
/// # Returns
/// A function that takes a `Span<'a>` and returns a `ParserResult<'a, (StrictSeparatedList<PO, SO>, TO)>`.
/// The `StrictSeparatedList` contains the parsed elements and separators, and the second item is the result of the termination parser.
#[inline]
pub fn strict_separated_list<
    'a,
    PO,
    PR: IntoParserResult<'a, PO>,
    SO,
    SR: IntoParserResult<'a, SO>,
    TO,
    TR: IntoParserResult<'a, TO>,
>(
    separator: impl Fn(Span<'a>) -> SR,
    parser: impl Fn(Span<'a>) -> PR,
    termination_parser: impl Fn(Span<'a>) -> TR,
    consume_termination_tokens: bool,
    require_one: bool,
) -> impl Fn(Span<'a>) -> ParserResult<'a, (StrictSeparatedList<PO, SO>, TO)> {
    move |mut input: Span<'a>| {
        let mut list = StrictSeparatedList::new();

        if require_one {
            match parser(input).into_parser_result() {
                Ok((rest, o)) => {
                    input = rest;
                    // Since the list is used correctly, unwrap is safe
                    list.push_value(o).expect("Failed to push value in StrictSeparatedList");
                }
                Err(e) => return Err(e),
            }
        };

        loop {
            // Try to parse the separator
            match separator(input).into_parser_result() {
                Ok((rest, sep)) => {
                    input = rest;

                    // Try to parse the next value
                    match parser(input).into_parser_result() {
                        Ok((rest, o)) => {
                            input = rest;
                            // Safe to unwrap as the parser enforces correct alternation
                            list.push_separator_value_pair(sep, o)
                                .expect("Failed to push separator-value pair in StrictSeparatedList");
                        }
                        Err(e) => {
                            // Attempt to parse the termination after separator
                            match termination_parser(input).into_parser_result() {
                                Ok((rest, o)) => {
                                    if consume_termination_tokens {
                                        return Ok((rest, (list, o)));
                                    } else {
                                        return Ok((input, (list, o)));
                                    }
                                }
                                Err(e2) => {
                                    // Termination parsing failed; return the error
                                    return Err(e2);
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    // Attempt to parse the termination
                    match termination_parser(input).into_parser_result() {
                        Ok((rest, o)) => {
                            if consume_termination_tokens {
                                return Ok((rest, (list, o)));
                            } else {
                                return Ok((input, (list, o)));
                            }
                        }
                        Err(e) => {
                            // Termination parsing failed; return the error
                            return Err(e);
                        }
                    }
                }
            }
        }
    }
}
