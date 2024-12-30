use fst::{EnumValue, Statement};
use parser_core::*;

use crate::{
    expression::parse_expression,
    utils::{opt, ws0},
};

use super::{semicolon::opt_semicolon, struct_stmt::parse_struct_block};

pub fn parse_enum_statement<'a>(input: Span<'a>) -> ParserResult<'a, Statement> {
    let (input, _) = parse_enum(input)?;
    let (input, _) = ws0(input);
    let (input, name) = parse_ident(input)?;
    let (input, _) = ws0(input);
    let (input, _) = parse_left_brace(input)?;
    let (input, (options, _)) = separated_list(
        (ws0, parse_comma).tuple(),
        preceded(
            ws0,
            (
                parse_ident.map(|s| s.to_string()),
                opt((
                    preceded((ws0, parse_left_paren).tuple(), parse_enum_arguments),
                    parse_struct_block.map(|v| EnumValue::Struct(v)),
                )
                    .alt())
                .map(|v| match v {
                    Some(type_) => type_,
                    None => EnumValue::Unit,
                }),
            )
                .tuple(),
        ),
        (ws0, parse_right_brace).tuple(),
        true,
        true,
        false,
    )(input)?;
    let (input, _) = opt_semicolon(input);
    Ok((
        input,
        Statement::Enum {
            name: name.to_string(),
            options,
        },
    ))
}

fn parse_enum_arguments<'a>(input: Span<'a>) -> ParserResult<'a, EnumValue> {
    separated_list(
        (ws0, parse_comma).tuple(),
        preceded(ws0, parse_expression),
        (ws0, parse_right_paren).tuple(),
        true,
        true,
        false,
    )
    .map(|v| EnumValue::Tuple(v.0))(input)
}
