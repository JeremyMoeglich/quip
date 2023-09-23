use ast::Expression;
use parser_core::*;

use crate::identifier::parse_identifier;

pub fn parse_variable<'a>(input: &Span<'a>) -> ParserResult<'a, Expression> {
    let (input, name) = parse_identifier(input)?;
    Ok((input, Expression::Variable(name)))
}
