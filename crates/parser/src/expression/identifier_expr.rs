use fst::Expression;
use parser_core::*;

pub fn parse_variable_expr<'a>(input: Span<'a>) -> ParserResult<'a, Expression> {
    let (input, name) = parse_ident(input)?;
    Ok((input, Expression::Variable { identifier: name.to_string() }))
}
