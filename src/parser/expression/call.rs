use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{
    ast::Expression,
    parser::utils::{ws, Span},
};

use super::parse_expression;

#[derive(Debug, Clone)]
enum CallGetEnum {
    Call(Vec<Expression>),
    Get(Expression),
}

pub fn parse_call_and_get(start_expr: Expression) -> impl Fn(Span) -> IResult<Span, Expression> {
    move |mut input: Span| {
        let mut tree = start_expr.clone();
        while let Ok((input2, args)) = alt((
            map(
                delimited(
                    tuple((tag("("), ws)),
                    separated_list0(tuple((ws, tag(","), ws)), parse_expression),
                    tuple((ws, tag(")"))),
                ),
                CallGetEnum::Call,
            ),
            map(
                delimited(
                    tuple((tag("["), ws)),
                    parse_expression,
                    tuple((ws, tag("]"))),
                ),
                CallGetEnum::Get,
            ),
        ))(input)
        {
            match args {
                CallGetEnum::Call(args) => tree = Expression::Call(Box::new(tree), args),
                CallGetEnum::Get(arg) => tree = Expression::Get(Box::new(tree), Box::new(arg)),
            }
            input = input2;
        }
        Ok((input, tree))
    }
}
