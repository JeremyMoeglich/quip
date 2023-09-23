use ast::Expression;
use parser_core::*;

use crate::{
    identifier::parse_identifier,
    utils::{ws0, ws_delimited, opt, separated_pair},
};

use super::parse_expression;

pub fn parse_object<'a>(input: &Span<'a>) -> ParserResult<'a, Expression> {
    let (input, name) = opt(parse_identifier)(input)?;
    let (input, _) = ws0(&input)?;
    let (input, _) = parse_LeftBrace(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, values) = separated_list0(
        ws_delimited(parse_Comma),
        separated_pair(parse_identifier, ws_delimited(parse_Colon), parse_expression),
    )(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, _) = parse_RightBrace(&input)?;
    Ok((input, Expression::Object(name, values)))
}
