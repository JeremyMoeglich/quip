use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{
    fst::{Statement, TypeExpression},
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{ws, ws1, Span},
};

use super::generic::parse_generics;

pub fn parse_struct(input: Span) -> IResult<Span, Statement> {
    let (input, _) = tag("struct")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, generics) = parse_generics(input)?;
    let (input, _) = ws(input)?;
    let (input, fields) = delimited(
        tuple((char('{'), ws)),
        separated_list0(
            tuple((ws, char(','), ws)),
            tuple((
                parse_identifier,
                map(
                    opt(tuple((ws, char(':'), ws, parse_type_expression))),
                    |v| match v {
                        Some((_, _, _, type_)) => type_,
                        None => TypeExpression::Infer,
                    },
                ),
            )),
        ),
        tuple((ws, opt(char(',')), ws, char('}'))),
    )(input)?;
    Ok((input, Statement::Struct(name, generics, fields)))
}
