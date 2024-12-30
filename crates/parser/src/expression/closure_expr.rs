use fst::{Closure, ClosureSignature, Expression};
use parser_core::*;

use crate::{
    function_parameters::parse_function_parameters,
    utils::{opt, ws0, ws1},
};

use super::parse_expression;

// Examples:
// a -> a + 2 // valid
// a, b -> a + b // valid
// a: Int -> a + 2 // invalid
// (a: Int) -> a + 2 // valid
// (a: Int, b) -> a + b // valid
// a -> Int

pub fn parse_closure_expr(input: Span) -> ParserResult<Expression> {
    let (input, params) = match parse_left_paren(input) {
        Ok((input, _)) => {
            let (input, _) = ws0(input);
            let (input, params) = parse_function_parameters(
                false,
                (ws0, parse_right_paren).tuple(),
            )(input)?;
            (input, params)
        }
        Err(left_paren_error) => {
            let (input, params) = parse_function_parameters(true, parse_arrow)(input)
                .map_err(|e| left_paren_error.accumulate(e))?;
            // All following errors are guaranteed to be better than the left paren error
            // because the left paren error is at offset 0, while all following errors are at offset 1 or more due to require_at_least_one
            // This means we don't need to accumulate anymore
            (input, params)
        }
    };
    let (input, _) = ws0(input);
    let (input, expr1) = parse_expression(input)?;
    let (input, expr2) = opt((ws1, parse_do, ws1, parse_expression).tuple().map(|v| v.3))(input);
    match expr2 {
        Some(expr2) => Ok((
            input,
            Expression::Closure {
                closure: Box::new(Closure {
                    body: expr2,
                    closure_signature: ClosureSignature {
                        params,
                        return_type: Some(expr1),
                    },
                }),
            },
        )),
        None => Ok((
            input,
            Expression::Closure {
                closure: Box::new(Closure {
                    body: expr1,
                    closure_signature: ClosureSignature {
                        params,
                        return_type: None,
                    },
                }),
            },
        )),
    }
}
