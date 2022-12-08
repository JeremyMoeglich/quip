use crate::fst::{Argument, Arguments};

use super::{
    common::comma_separated,
    core::{ParseResult, Parser, TokenSlice},
    expression::parse_expression,
};

pub fn parse_arguments(input: TokenSlice) -> ParseResult<Arguments> {
    comma_separated(parse_expression)
        .map_result(|args| {
            args.iter()
                .map(|arg| Argument::new(arg.0, arg.1, arg.2))
                .collect()
        })
        .parse(input)
}
