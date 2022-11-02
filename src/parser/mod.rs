mod assignment;
pub mod expression;
mod identifier;
mod statement;
mod utils;
mod function;
mod block;
mod if_statement;
mod declaration;

use nom::{multi::many0, IResult};

use crate::{
    ast::{CodeBlock},
    parser::utils::Span,
};

use self::{statement::parse_statement, utils::new_span};

pub fn parse_code(input: Span) -> IResult<Span, CodeBlock> {
    many0(parse_statement)(input)
}

pub fn simple_parse(input: &str) -> Result<CodeBlock, nom::Err<nom::error::Error<Span>>> {
    let input = new_span(input);
    let iresult = parse_code(input);
    match iresult {
        Ok((_, expression)) => Ok(expression),
        Err(err) => Err(err),
    }
}
