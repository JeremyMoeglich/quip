use crate::{expression::parse_expression, utils::ws0};
use fst::{Expression, Statement};
use parser_core::*;

use super::semicolon::opt_semicolon;

pub fn parse_struct_block<'a>(input: Span<'a>) -> ParserResult<'a, Vec<(String, Expression)>> {
    let (input, _) = parse_left_brace(input)?;
    let (input, (fields, _)) = separated_list(
        (ws0, parse_comma).tuple(),
        (ws0, parse_ident, ws0, parse_colon, ws0, parse_expression)
            .tuple()
            .map(|(_, ident, _, _, _, expr)| (ident.to_string(), expr)),
        (ws0, parse_right_brace).tuple(),
        true,
        true,
        false,
    )(input)?;
    Ok((input, fields))
}
pub fn parse_struct_statement<'a>(input: Span<'a>) -> ParserResult<'a, Statement> {
    let (input, _) = parse_struct(input)?;
    let (input, _) = ws0(input);
    let (input, name) = parse_ident(input)?;
    let (input, _) = ws0(input);

    let (input, fields) = parse_struct_block(input)?;
    let (input, _) = opt_semicolon(input);

    Ok((
        input,
        Statement::Struct {
            name: name.to_string(),
            fields,
        },
    ))
}
