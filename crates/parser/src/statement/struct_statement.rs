

use crate::{
    ast::{Statement, TypeExpression},
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{ws0, ws1, Span},
};

use super::generic::parse_generics;

pub fn parse_struct(input: Span) -> IResult<Span, Statement> {
    let (input, _) = tag("struct")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = ws0(input)?;
    let (input, generics) = parse_generics(input)?;
    let (input, _) = ws0(input)?;
    let (input, fields) = delimited(
        tuple((char('{'), ws0)),
        separated_list0(
            tuple((ws0, char(','), ws0)),
            tuple((
                parse_identifier,
                map(
                    opt(tuple((ws0, char(':'), ws0, parse_type_expression))),
                    |v| match v {
                        Some((_, _, _, type_)) => type_,
                        None => TypeExpression::Infer,
                    },
                ),
            )),
        ),
        tuple((ws0, opt(char(',')), ws0, char('}'))),
    )(input)?;
    Ok((input, Statement::Struct(name, generics, fields)))
}
