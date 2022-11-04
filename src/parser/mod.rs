mod assignment;
mod block;
mod declaration;
pub mod expression;
mod function;
mod identifier;
mod if_statement;
mod statement;
mod utils;

use nom::{multi::many0, IResult};

use crate::{ast::CodeBlock, parser::utils::Span};

use self::{statement::parse_statement, utils::{new_span, ws}};

pub fn parse_code(input: Span) -> IResult<Span, CodeBlock> {
    let (input, out) = many0(parse_statement)(input)?;
    let (input, _) = ws(input)?;
    Ok((input, out))
}

pub fn simple_parse(input: &str) -> Result<CodeBlock, nom::Err<nom::error::Error<Span>>> {
    let input = new_span(input);
    let iresult = parse_code(input);
    match iresult {
        Ok((input, expression)) => match input.fragment() {
            &"" => Ok(expression),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Eof,
            ))),
        },
        Err(err) => Err(err),
    }
}
