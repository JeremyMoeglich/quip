use crate::{
    expression::parse_expression,
    utils::{opt, ws0},
};
use fst::{PropertySignature, Signature, Statement};
use parser_core::*;

use super::{function_stmt::parse_fn_signature, semicolon::opt_semicolon};

pub fn parse_trait_statement<'a>(input: Span<'a>) -> ParserResult<'a, Statement> {
    let (input, _) = parse_trait(input)?;
    let (input, _) = ws0(input);
    let (input, name) = parse_ident(input)?;
    let (input, _) = ws0(input);
    let (input, _) = parse_left_brace(input)?;
    let (input, _) = ws0(input);
    let (input, (signatures, _)) =
        separated_list(ws0, parse_signature, parse_right_brace, true, true, false)(input)?;
    let (input, _) = opt_semicolon(input);
    Ok((
        input,
        Statement::Trait {
            name: name.to_string(),
            signatures,
        },
    ))
}

fn parse_signature<'a>(input: Span<'a>) -> ParserResult<'a, Signature> {
    let (input, signature) = (
        parse_fn_signature.map(|v| Signature::Function(v)),
        parse_property_signature,
    )
        .alt()(input)?;
    let (input, _) = ws0(input);
    let (input, _) = opt(parse_semicolon)(input);
    let (input, _) = ws0(input);
    Ok((input, signature))
}

fn parse_property_signature<'a>(input: Span<'a>) -> ParserResult<'a, Signature> {
    let (input, (token, source_span)) = input.take_token();
    match token.delocate() {
        Some(Token::Ident(name)) => {
            let (input, _) = ws0(input);
            let (input, _) = parse_colon(input)?;
            let (input, _) = ws0(input);
            let (input, return_type) = parse_expression(input)?;
            Ok((
                input,
                Signature::Property(PropertySignature {
                    name: name.to_string(),
                    value_type: return_type,
                    mutable: false,
                }),
            ))
        }
        Some(Token::Mut) => {
            let (input, _) = ws0(input);
            let (input, name) = parse_ident(input)?;
            let (input, _) = ws0(input);
            let (input, _) = parse_colon(input)?;
            let (input, _) = ws0(input);
            let (input, return_type) = parse_expression(input)?;
            Ok((
                input,
                Signature::Property(PropertySignature {
                    name: name.to_string(),
                    value_type: return_type,
                    mutable: true,
                }),
            ))
        }
        _ => Err(token.as_parser_error(TokenKind::Ident | TokenKind::Mut, source_span)),
    }
}
