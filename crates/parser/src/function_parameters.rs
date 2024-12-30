use crate::{
    expression::parse_expression,
    utils::{opt, ws0},
    variable_creation::parse_variable_creation,
};
use fst::{Expression, VariableCreation};
use parser_core::*;

fn parse_function_parameter<'a>(
    input: Span<'a>,
) -> ParserResult<'a, (VariableCreation, Option<Expression>)> {
    let (input, creation) = parse_variable_creation(input)?;
    let (input, value_type) = opt((ws0, parse_colon, ws0, parse_expression)
        .tuple()
        .map(|v| v.3))(input);
    Ok((input, (creation, value_type)))
}

pub fn parse_function_parameters<'a, TO>(
    require_at_least_one: bool,
    termination_parser: impl Fn(Span<'a>) -> ParserResult<'a, TO>,
) -> impl Fn(Span<'a>) -> ParserResult<'a, Vec<(VariableCreation, Option<Expression>)>> {
    move |input| {
        let (input, (params, _)) = separated_list(
            (ws0, parse_comma).tuple(),
            preceded(ws0, parse_function_parameter),
            &termination_parser,
            true,
            true,
            require_at_least_one,
        )(input)?;

        Ok((input, params))
    }
}
