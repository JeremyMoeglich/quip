use crate::{
    block::parse_block, expression::parse_expression,
    function_parameters::parse_function_parameters, utils::{ws0, ws1},
};
use fst::{Closure, ClosureSignature, Expression, FunctionSignature, Statement};
use parser_core::*;

use super::semicolon::opt_semicolon;

pub fn parse_function_statement<'a>(input: Span<'a>) -> ParserResult<'a, Statement> {
    let (input, signature) = parse_fn_signature(input)?;

    let (input, _) = ws0(input);
    let (input, code) = parse_block(input)?;

    let (input, _) = opt_semicolon(input);

    Ok((
        input,
        Statement::Function {
            name: signature.name,
            closure: Closure {
                closure_signature: signature.closure_signature,
                body: Expression::Block {
                    environment: None,
                    block: code,
                },
            },
        },
    ))
}

pub fn parse_fn_signature<'a>(input: Span<'a>) -> ParserResult<'a, FunctionSignature> {
    let (input, _) = parse_fn(input)?;
    let (input, _) = ws1(input)?;
    let (input, name) = parse_ident(input)?;
    let (input, _) = ws0(input);

    let (input, _) = parse_left_paren(input)?;
    let (input, params) = parse_function_parameters(false, parse_right_paren)(input)?;

    let (input, _) = ws0(input);

    if let Ok((input, _)) = parse_arrow(input) {
        let (input, _) = ws0(input);
        let (input, return_type) = parse_expression(input)?;
        Ok((
            input,
            FunctionSignature {
                name: name.to_string(),
                closure_signature: ClosureSignature {
                    params,
                    return_type: Some(return_type),
                },
            },
        ))
    } else {
        Ok((
            input,
            FunctionSignature {
                name: name.to_string(),
                closure_signature: ClosureSignature {
                    params,
                    return_type: None,
                },
            },
        ))
    }
}
