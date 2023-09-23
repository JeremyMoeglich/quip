use ast::Expression;
use parser_core::*;

use crate::utils::ws0;

use super::parse_expression;

pub fn parse_list<'a>(input: &Span<'a>) -> ParserResult<'a, Expression> {
    let (input, value) = delimited(
        (parse_LeftBracket, MapParser::map(ws0, |_| ())).tuple(),
        separated_list0(
            (ws0, parse_Comma, ws0).tuple(),
            parse_expression,
        ),
        (parse_RightBracket, ws0).tuple(),
    )(&input)?;
    Ok((input, Expression::List(value)))
}
