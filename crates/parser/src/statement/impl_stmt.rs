use crate::{block::parse_block, utils::{opt, ws0, ws1}};
use fst::{Expression, Statement};
use parser_core::*;

use super::semicolon::opt_semicolon;

pub fn parse_impl_statement<'a>(input: Span<'a>) -> ParserResult<'a, Statement> {
    let (input, _) = parse_impl(input)?;
    let (input, _) = ws0(input);
    let (input, trait_name) = opt((parse_ident, ws1, parse_for)
        .tuple()
        .map(|v| v.0.to_string()))(input);
    let (input, identifier) = parse_ident(input)?;
    let (input, _) = ws0(input);
    let (input, statements) = parse_block(input)?;
    let (input, _) = opt_semicolon(input);

    Ok((
        input,
        Statement::Impl {
            target: identifier.to_string(),
            implemented: trait_name.map(|v| Expression::Variable {
                identifier: v.to_string(),
            }),
            statements,
        },
    ))
}
