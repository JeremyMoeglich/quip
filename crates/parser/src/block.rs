use fst::Statement;
use parser_core::*;

use crate::{separated_list::{self, parser::trailing_separated_list}, statement::parse_statement};

use super::utils::ws0;

pub fn parse_block<'a>(input: Span<'a>) -> ParserResult<'a, Vec<Statement>> {
    let (input, _) = parse_left_brace(input)?;
    let (input, _) = ws0(input);
    let (input, statements) = parse_righthand_block(input)?;
    Ok((input, statements))
}

pub fn parse_righthand_block<'a>(input: Span<'a>) -> ParserResult<'a, Vec<Statement>> {
    let (input, (statements, _)) = trailing_separated_list(
        ws0,
        parse_statement,
        parse_right_brace,
        true,
        false,
    )(input)?;
    Ok((input, statements))
}
