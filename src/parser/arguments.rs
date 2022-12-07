use crate::fst::{Argument, Arguments};

use super::{
    expression::parse_expression,
    core::{comma_separated, ParseResult, TokenSlice},
};

pub fn parse_arguments(input: TokenSlice) -> ParseResult<Arguments> {
    let (input, values) = comma_separated(parse_expression)(input)?;
    let args = values
        .into_iter()
        .map(|(expr, space, second_space)| Argument::new(expr, space, second_space))
        .collect();
    Ok((input, args))
}
