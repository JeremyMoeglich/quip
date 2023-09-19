

use crate::{
    ast::Statement,
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{ws0, ws1, ws_delimited, Span},
};

use super::generic::parse_generics;

pub fn parse_enum(input: Span) -> IResult<Span, Statement> {
    let (input, _) = tag("enum")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = ws0(input)?;
    let (input, generics) = parse_generics(input)?;
    let (input, _) = ws0(input)?;
    let (input, options) = delimited(
        tuple((char('{'), ws0)),
        separated_list0(
            tuple((ws0, char(','), ws0)),
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
        tuple((ws0, opt(char(',')), ws0, char('}'))),
    )(input)?;
    Ok((input, Statement::Enum(name, generics, options)))
}
