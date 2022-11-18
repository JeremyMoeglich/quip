use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{
    fst::Statement,
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{ws, ws1, ws_delimited, Span},
};

use super::generic::parse_generics;

pub fn parse_enum(input: Span) -> IResult<Span, Statement> {
    let (input, _) = tag("enum")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, generics) = parse_generics(input)?;
    let (input, _) = ws(input)?;
    let (input, options) = delimited(
        tuple((char('{'), ws)),
        separated_list0(
            tuple((ws, char(','), ws)),
            tuple((
                parse_identifier,
                map(
                    opt(delimited(
                        ws_delimited(char('(')),
                        separated_list0(ws_delimited(char(',')), parse_type_expression),
                        ws_delimited(char(')')),
                    )),
                    |v| match v {
                        Some(type_) => type_,
                        None => vec![],
                    },
                ),
            )),
        ),
        tuple((ws, opt(char(',')), ws, char('}'))),
    )(input)?;
    Ok((input, Statement::Enum(name, generics, options)))
}
