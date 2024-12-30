use fst::Statement;
use parser_core::*;

use crate::{block::parse_block, utils::ws0};

use super::semicolon::opt_semicolon;

pub fn parse_module_statement<'a>(input: Span<'a>) -> ParserResult<'a, Statement> {
    let (input, _) = parse_mod(input)?;
    let (input, _) = ws0(input);
    let (input, name) = parse_ident(input)?;
    let (input, _) = ws0(input);
    let (input, statements) = parse_block(input)?;
    let (input, _) = opt_semicolon(input);
    Ok((
        input,
        Statement::Module {
            name: name.to_string(),
            statements,
        },
    ))
}
