use nom::IResult;

use crate::{fst::Expression, identifier::parse_identifier, utils::Span};

pub fn parse_variable(input: Span) -> IResult<Span, Expression> {
    let (input, name) = parse_identifier(input)?;
    Ok((input, Expression::Variable(name)))
}
