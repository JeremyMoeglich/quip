use ast::StatementInner;
use parser_core::*;

use crate::{expression::parse_expression, utils::ws0};

pub fn parse_assignment<'a>(input: &Span<'a>) -> ParserResult<'a, StatementInner> {
    let (input, to_change) = parse_expression(input)?;
    let (input, _) = ws0(&input)?;
    let (input, _) = parse_Assign(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, expression) = parse_expression(&input)?;
    Ok((
        input,
        StatementInner::Assignment {
            left: to_change,
            right: expression,
        },
    ))
}
