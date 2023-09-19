

use crate::{
    ast::TypeExpression,
    identifier::parse_identifier,
    utils::{ws0, ws_delimited, Span},
};

use super::parse_type_expression;

pub fn parse_type_object(input: Span) -> IResult<Span, TypeExpression> {
    let (input, _) = tuple((ws0, char('{'), ws0))(input)?;
    let (input, parameters) = separated_list0(
        ws_delimited(char(',')),
        tuple((
            parse_identifier,
            ws_delimited(char(':')),
            parse_type_expression,
        )),
    )(input)?;
    let (input, _) = tuple((ws0, char('}'), ws0))(input)?;
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
