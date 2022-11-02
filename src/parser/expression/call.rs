use nom::{
    bytes::complete::tag,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{
    ast::Expression,
    parser::utils::{ws, Span},
};

use super::parse_expression;

pub fn parse_call(start_expr: Expression) -> impl Fn(Span) -> IResult<Span, Expression> {
    move |mut input: Span| {
        let mut tree = start_expr.clone();
        while let Ok((input2, args)) = delimited(
            tuple((tag("("), ws)),
            separated_list0(tuple((ws, tag(","), ws)), parse_expression),
            tuple((ws, tag(")"))),
        )(input)
        {
            tree = Expression::Call(Box::new(tree), args);
            input = input2;
        }
        Ok((input, tree))
    }
}
