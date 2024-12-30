#![feature(closure_lifetime_binder)]
#![feature(impl_trait_in_assoc_type)]
#![feature(const_trait_impl)]

mod block;
mod destructure;
mod error;
pub mod expression;
mod function_parameters;
mod statement;
mod utils;
mod variable_creation;
mod whitespace;
mod separated_list;

use self::{statement::parse_statement, utils::ws0};
use fst::Statement;
use error::create_fancy_error;
use parser_core::*;

pub mod core {
    pub use parser_core::*;
}

pub fn parse_file<'a>(input: Span<'a>) -> ParserResult<'a, Vec<Statement>> {
    let (input, _) = ws0(input);
    let (input, out) =
        aggressive_many0((parse_statement, ws0).tuple().map(|(stmt, _)| stmt))(input)?;
    Ok((input, out))
}

pub fn simple_parse(code: &str) -> Result<Vec<Statement>, String> {
    let tokens = tokenize(code);
    let input = create_span(&tokens);
    let result = parse_file(input);
    match result {
        Ok((input2, statements)) => match input2.tokens.len() {
            0 => Ok(statements),
            _ => unreachable!(),
        },
        Err(err) => Err(create_fancy_error(&code, err)),
    }
}
