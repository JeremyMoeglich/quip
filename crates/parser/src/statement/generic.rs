

use crate::{
    ast::TypeExpression,
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{ws, ws_delimited, Span},
};

fn parse_single_generic(input: Span) -> IResult<Span, (String, TypeExpression)> {
    let (input, name) = parse_identifier(input)?;
    let (input, type_) = map(
        opt(tuple((ws, char(':'), ws, parse_type_expression))),
        |v| match v {
            Some((_, _, _, type_)) => type_,
            None => TypeExpression::Infer,
        },
    )(input)?;
    Ok((input, (name, type_)))
}

pub fn parse_generics(input: Span) -> IResult<Span, Vec<(String, TypeExpression)>> {
    match opt(delimited(
        char('<'),
        separated_list0(ws_delimited(char(',')), parse_single_generic),
        char('>'),
    ))(input)?
    {
        (input, Some(generics)) => Ok((input, generics)),
        (input, None) => Ok((input, vec![])),
    }
}
