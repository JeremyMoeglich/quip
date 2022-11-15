pub mod fst;
mod block;
mod error;
pub mod expression;
mod identifier;
mod statement;
mod type_expression;
mod utils;
mod lexer;

use nom::{multi::many0, IResult};

use crate::utils::Span;

use self::{
    fst::CodeBlock,
    error::{create_fancy_error, create_fancy_error_span},
    statement::parse_statement,
    utils::{new_span, ws},
};

pub fn parse_code(input: Span) -> IResult<Span, CodeBlock> {
    let (input, out) = many0(parse_statement)(input)?;
    let (input, _) = ws(input)?;
    Ok((input, out))
}

pub fn simple_parse(code: &str) -> Result<CodeBlock, String> {
    let input = new_span(code);
    let iresult = parse_code(input);
    match iresult {
        Ok((input2, expression)) => match input2.fragment() {
            &"" => Ok(expression),
            _ => Err(create_fancy_error_span(&code, input2)),
        },
        Err(err) => Err(create_fancy_error(&code, err)),
    }
}
