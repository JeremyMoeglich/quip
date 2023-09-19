

use crate::{
    ast::Statement,
    expression::parse_expression,
    utils::{ws0, Span},
};

pub fn parse_assignment(input: Span) -> IResult<Span, Statement> {
    let (input, to_change) = parse_expression(input)?;
    let (input, _) = ws0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = ws0(input)?;
    let (input, expression) = parse_expression(input)?;
    Ok((input, Statement::Assignment(to_change, expression)))
}
