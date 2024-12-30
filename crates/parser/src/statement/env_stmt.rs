use crate::{expression::parse_expression, utils::ws0};
use fst::Statement;
use parser_core::*;

use super::semicolon::require_semicolon;

pub fn parse_env_statement<'a>(input: Span<'a>) -> ParserResult<'a, Statement> {
    let (input, _) = parse_use_env(input)?;
    let (input, _) = ws0(input);
    let (input, name) = parse_expression(input)?;
    let (input, _) = require_semicolon(input)?;
    Ok((input, Statement::Env(name)))
}