use fst::Statement;
use parser_core::*;

use crate::{
    destructure::parse_mutable_extract,
    expression::parse_expression,
    utils::{ws0, ws1},
};

use super::semicolon::{opt_semicolon, require_semicolon};

pub fn parse_import_statement<'a>(input: Span<'a>) -> ParserResult<'a, Statement> {
    let (input, _) = parse_import(input)?;
    let (input, _) = ws0(input);
    let (input, import_expression) = parse_expression(input)?;

    match (ws1, parse_as).tuple()(input) {
        Ok((input, _)) => {
            let (input, _) = ws1(input)?;
            let (input, extract) = parse_mutable_extract(input)?;
            let (input, _) = opt_semicolon(input);
            Ok((
                input,
                Statement::Import {
                    importable: import_expression,
                    extract: Some(extract),
                },
            ))
        }
        Err(_) => {
            let (input, _) = require_semicolon(input)?;

            Ok((
                input,
                Statement::Import {
                    importable: import_expression,
                    extract: None,
                },
            ))
        }
    }
}
