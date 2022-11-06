use nom::{character::complete::char, multi::separated_list0, sequence::tuple, IResult};

use crate::parser::{
    ast::TypeExpression,
    identifier::parse_identifier,
    utils::{ws, ws_delimited, Span},
};

use super::parse_type_expression;

pub fn parse_type_object(input: Span) -> IResult<Span, TypeExpression> {
    let (input, _) = tuple((ws, char('{'), ws))(input)?;
    let (input, parameters) = separated_list0(
        ws_delimited(char(',')),
        tuple((
            parse_identifier,
            ws_delimited(char(':')),
            parse_type_expression,
        )),
    )(input)?;
    let (input, _) = tuple((ws, char('}'), ws))(input)?;
    Ok((
        input,
        TypeExpression::Object(
            parameters
                .iter()
                .map(|(name, _, type_)| (name.clone(), type_.clone()))
                .collect::<Vec<_>>(),
        ),
    ))
}
