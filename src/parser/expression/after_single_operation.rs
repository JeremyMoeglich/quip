use nom::{branch::alt, bytes::complete::tag, combinator::value, sequence::tuple, IResult};

use crate::{
    ast::{Expression, SingleOperation},
    parser::utils::{ws, Span},
};

use super::{parse_expression_with_rule, ExpressionParseRules};

pub fn parse_after_single_operation(
    rules: ExpressionParseRules,
) -> impl Fn(Span) -> IResult<Span, Expression> {
    move |input: Span| {
        let (input, (expression, _, single_operation)) = tuple((
            parse_expression_with_rule(rules.with_after_single_operation(false)),
            ws,
            alt((
                value(SingleOperation::Panic, tag("?!")),
                value(SingleOperation::ErrorUnwrap, tag("?")),
            )),
        ))(input)?;
        Ok((
            input,
            Expression::SingleOperation(single_operation, Box::new(expression)),
        ))
    }
}
