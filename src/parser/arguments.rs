use crate::fst::{Argument, Arguments};

use super::{
    common::comma_separated,
    core::{ParseResult, Parser, TokenSlice},
    expression::parse_expression,
};

pub fn parse_arguments(input: TokenSlice) -> ParseResult<Arguments> {
    comma_separated(parse_expression)
        .map_result(|args| {
            args.into_iter()
                .map(|(expr, space1, space2)| Argument::new(expr, space1, space2))
                .collect()
        })
        .parse(input)
}
