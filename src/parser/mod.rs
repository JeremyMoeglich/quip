pub mod expression;
mod identifier;
mod utils;

use nom::IResult;

use crate::{ast::Expression, parser::utils::Span};

use self::utils::new_span;

pub fn parse(input: Span) -> IResult<Span, Expression> {
    expression::parse_expression(input)
}

pub fn simple_parse(input: &str) -> Result<Expression, nom::Err<nom::error::Error<Span>>> {
    let input = new_span(input);
    let iresult = parse(input);
    match iresult {
        Ok((_, expression)) => Ok(expression),
        Err(err) => Err(err),
    }
}
