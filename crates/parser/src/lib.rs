#![feature(return_position_impl_trait_in_trait)]

mod block;
mod error;
pub mod expression;
mod identifier;
mod statement;
mod type_expression;
mod utils;

use self::{statement::parse_statement, utils::ws0};
use ast::CodeBlock;
use error::{create_fancy_error_span, create_fancy_error};
use parser_core::*;

pub fn parse_code<'a>(input: &Span<'a>) -> ParserResult<'a, CodeBlock, TokenParserError> {
    let (input, out) = many0(parse_statement)(input)?;
    let (input, _) = ws0(&input)?;
    Ok((input, out))
}

pub fn simple_parse(code: &str) -> Result<CodeBlock, String> {
    let tokens = tokenize(code);
    let input = create_span(&tokens);
    let iresult = parse_code(&input);
    match iresult {
        Ok((input2, expression)) => match input2.fragment() {
            &"" => Ok(expression),
            _ => Err(create_fancy_error_span(&code, input2)),
        },
        Err(err) => Err(create_fancy_error(&code, err)),
    }
}
