

use crate::{
    ast::{Statement, TypeExpression},
    expression::parse_expression,
    identifier::parse_identifier,
    type_expression::parse_type_expression,
    utils::{ws, ws1, Span},
};

pub fn parse_declaration(input: Span) -> IResult<Span, Statement> {
    let (input, _) = tag("let")(input)?;
    let (input, _) = ws1(input)?;
    let (input, mutable) = map(opt(tuple((tag("mut"), ws1))), |v| match v {
        Some(_) => true,
        None => false,
    })(input)?;
    let (input, identifier) = parse_identifier(input)?;
    let (input, type_) = map(
        opt(tuple((ws, char(':'), ws, parse_type_expression))),
        |v| match v {
            Some((_, _, _, type_)) => type_,
            None => TypeExpression::Infer,
        },
    )(input)?;
    let (input, _) = ws(input)?;
    let (input, expression_opt) =
        map(opt(tuple((char('='), ws, parse_expression))), |v| match v {
            Some((_, _, expression)) => Some(expression),
            None => None,
        })(input)?;
    Ok((
        input,
        Statement::Declaration((identifier, type_), mutable, expression_opt),
    ))
}
