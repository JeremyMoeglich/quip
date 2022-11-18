use nom::{character::complete::char, sequence::tuple, IResult};

use crate::{
    fst::TypeExpression,
    utils::{ws, Span},
};

pub fn parse_type_array<'a, F>(
    mut start_parser: F,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, TypeExpression>
where
    F: FnMut(Span<'a>) -> IResult<Span<'a>, TypeExpression>,
{
    move |input| {
        let (mut input, mut expression) = start_parser(input)?;
        while let Ok((new_input, _)) = tuple((ws, char('['), ws, char(']'), ws))(input) {
            input = new_input;
            expression = TypeExpression::Array(Box::new(expression));
        }
        Ok((input, expression))
    }
}
