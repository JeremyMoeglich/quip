use crate::{
    block::parse_block,
    expression::parse_expression,
    identifier::parse_identifier,
    utils::{opt, ws0, ws1, ws_delimited},
};
use ast::{Expression, StatementInner, TypedIdentifier};
use parser_core::*;

use super::generic::parse_generics;

pub fn parse_function<'a>(input: &Span<'a>) -> ParserResult<'a, StatementInner> {
    let (input, _) = parse_Fn(input)?;
    let (input, _) = ws1(&input)?;
    let (input, name) = parse_identifier(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, generics) = parse_generics(&input)?;
    let (input, _) = ws0(&input)?;

    let (input, params) = delimited(
        (parse_LeftParen, ws0).tuple(),
        separated_list0(
            (ws0, parse_Comma, ws0).tuple(),
            (
                parse_identifier,
                opt((ws0, parse_Colon, ws0, parse_expression).tuple()).map(|v| {
                    match v {
                        Some((_, _, _, type_)) => type_,
                        None => Expression::Infer,
                    }
                }),
            )
                .tuple(),
        ),
        (ws0, parse_RightParen).tuple(),
    )(&input)?;

    let (input, _) = ws0(&input)?;

    let (input, return_type) = opt(preceded(
        ws_delimited(parse_Arrow),
        parse_expression,
    ))
    .map(|v| match v {
        Some(type_) => type_,
        None => Expression::Infer,
    })(&input)?;

    let (input, _) = ws0(&input)?;
    let (input, code) = parse_block(&input)?;

    Ok((
        input,
        StatementInner::Function {
            name,
            generics,
            params: params
                .iter()
                .map(|(name, type_)| TypedIdentifier {
                    expression: type_.clone(),
                    identifier: name.clone(),
                })
                .collect::<Vec<_>>(),
            ret_type: return_type,
            body: code,
        },
    ))
}
