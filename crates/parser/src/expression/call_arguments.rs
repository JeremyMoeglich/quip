use fst::CallArguments;
use parser_core::*;

use crate::utils::{separated_pair, ws0, ws_delimited};

use super::parse_expression;

pub fn parse_call_arguments(input: Span) -> ParserResult<CallArguments> {
    (parse_positional_call_arguments, parse_named_call_arguments).alt()(input)
}

fn parse_positional_call_arguments(input: Span) -> ParserResult<CallArguments> {
    let (input, _) = parse_left_paren(input)?;
    let (input, (expressions, _)) = separated_list(
        (ws0, parse_comma).tuple(),
        (ws0, parse_expression).tuple().map(|(_, expr)| expr),
        (ws0, parse_right_paren).tuple(),
        true,
        true,
        false,
    )(input)?;
    Ok((input, CallArguments::Positional(expressions)))
}

fn parse_named_call_arguments(input: Span) -> ParserResult<CallArguments> {
    let (input, _) = parse_left_brace(input)?;
    let (input, (values, _)) = separated_list(
        (ws0, parse_comma).tuple(),
        (
            ws0,
            parse_ident.map(|s| s.to_string()),
            ws0,
            parse_colon,
            ws0,
            parse_expression,
        )
            .tuple(),
        (ws0, parse_right_brace).tuple(),
        true,
        true,
        false,
    )(input)?;
    Ok((input, CallArguments::Named(values)))
}
