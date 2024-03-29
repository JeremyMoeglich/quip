use crate::{
    expression::parse_expression,
    identifier::parse_identifier,
    utils::{opt, ws0, ws1},
};
use ast::{Expression, StatementInner, TypedIdentifier};
use parser_core::*;

pub fn parse_declaration<'a>(input: &Span<'a>) -> ParserResult<'a, StatementInner> {
    let (input, _) = parse_Let(input)?;
    let (input, _) = ws1(&input)?;

    let (input, mutable) =
        opt((parse_Mut, ws1).tuple()).map(|v| v.is_some())(&input)?;

    let (input, identifier) = parse_identifier(&input)?;

    let (input, type_) = opt((ws0, parse_Colon, ws0, parse_expression).tuple())
        .map(|v| match v {
            Some((_, _, _, type_)) => type_,
            None => Expression::Infer,
        })(&input)?;

    let (input, _) = ws0(&input)?;

    let (input, expression_opt) =
        opt((parse_Assign, ws0, parse_expression).tuple()).map(|v| match v {
            Some((_, _, expression)) => Some(expression),
            None => None,
        })(&input)?;

    Ok((
        input,
        StatementInner::Declaration {
            mutable,
            identifier: TypedIdentifier {
                identifier,
                expression: type_,
            },
            initializer: expression_opt,
        },
    ))
}
