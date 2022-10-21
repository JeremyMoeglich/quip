use nom::{bytes::complete::tag, multi::separated_list0, sequence::delimited, IResult};

use crate::{
    ast::Expression,
    parser::utils::{ws, Span},
};

use super::{parse_expression, parse_expression_with_rule, ExpressionParseRules};

pub fn parse_call(rules: ExpressionParseRules) -> impl Fn(Span) -> IResult<Span, Expression> {
    move |input: Span| {
        let (input, left_expr) = parse_expression_with_rule(rules.with_call(false))(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, args) = separated_list0(delimited(ws, tag(","), ws), parse_expression)(input)?;
        let (input, _) = tag(")")(input)?;
        Ok((input, Expression::Call(Box::new(left_expr), args)))
    }
}
