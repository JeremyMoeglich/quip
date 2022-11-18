use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::{
    fst::Statement,
    identifier::parse_identifier,
    utils::{ws, ws1, Span},
};

use super::parse_statement;

pub fn parse_impl(input: Span) -> IResult<Span, Statement> {
    let (input, _) = tag("impl")(input)?;
    let (input, _) = ws1(input)?;
    let (input, trait_name) = map(
        opt(tuple((parse_identifier, ws1, tag("for")))),
        |v| match v {
            Some((trait_name, _, _)) => Some(trait_name),
            None => None,
        },
    )(input)?;
    let (input, identifier) = parse_identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('{')(input)?;
    let (input, _) = ws(input)?;
    let (input, statements) = many0(parse_statement)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('}')(input)?;
    Ok((input, Statement::Impl(identifier, trait_name, statements)))
}
