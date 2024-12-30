use crate::{
    destructure::parse_mutable_destructure,
    expression::parse_expression,
    utils::{opt, ws0, ws1},
};
use fst::{VariableCreation, Expression};
use parser_core::*;

pub fn parse_declaration_expr<'a>(input: Span<'a>) -> ParserResult<'a, Expression> {
    let (input, _) = parse_let(input)?;
    let (input, _) = ws0(input);

    let (input, declared) = parse_variable_creation(input)?;

    let (input, value_type) =
        opt((ws0, parse_colon, ws0, parse_expression).tuple()).map(|v| match v {
            Some((_, _, _, type_)) => Some(type_),
            None => None,
        })(input);

    let (input, _) = ws0(input);

    let (input, expression_opt) =
        opt((parse_assignment, ws0, parse_expression).tuple()).map(|v| match v {
            Some((_, _, expression)) => Some(expression),
            None => None,
        })(input);

    Ok((
        input,
        Expression::Declaration {
            creation: declared,
            value_type: value_type.map(|v| Box::new(v)),
            initializer: expression_opt.map(|v| Box::new(v)),
        },
    ))
}

fn parse_variable_creation<'a>(input: Span<'a>) -> ParserResult<'a, VariableCreation> {
    (
        (opt((parse_mut, ws1).tuple()), parse_ident)
            .tuple()
            .map(|(mutable, ident)| VariableCreation::Identifier {
                name: ident.to_string(),
                mutable: mutable.is_some(),
            }),
        parse_mutable_destructure.map(|destructure| VariableCreation::Destructure { destructure }),
    )
        .alt()(input)
}
