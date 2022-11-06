mod array;
mod object;

use nom::{
    branch::alt, character::complete::char, combinator::map, multi::separated_list1,
    sequence::delimited, IResult,
};

use crate::parser::{ast::TypeExpression, utils::Span};

use self::{array::parse_type_array, object::parse_type_object};

use super::{
    identifier::parse_identifier,
    utils::{acond, ws_delimited},
};

pub fn parse_type_expression_with_rules(
    rules: TypeExpressionParseRules,
) -> impl Fn(Span) -> IResult<Span, TypeExpression> {
    move |input| {
        alt((
            parse_type_array(map(
                delimited(
                    ws_delimited(char('[')),
                    separated_list1(ws_delimited(char(',')), parse_type_expression),
                    ws_delimited(char(']')),
                ),
                |types| TypeExpression::Tuple(types),
            )),
            parse_type_object,
            acond(
                rules.allow_union,
                map(
                    separated_list1(
                        ws_delimited(char('|')),
                        parse_type_expression_with_rules(rules.with_union(false)),
                    ),
                    |expressions| match expressions.len() {
                        0 => unreachable!(),
                        1 => expressions.into_iter().next().unwrap(),
                        _ => TypeExpression::Union(expressions),
                    },
                ),
            ),
            acond(
                rules.allow_intersection,
                map(
                    separated_list1(
                        ws_delimited(char('&')),
                        parse_type_expression_with_rules(rules.with_intersection(false)),
                    ),
                    |expressions| TypeExpression::Intersection(expressions),
                ),
            ),
            parse_type_array(map(parse_identifier, |identifier| {
                TypeExpression::Variable(identifier)
            })),
        ))(input)
    }
}

pub fn parse_type_expression(input: Span) -> IResult<Span, TypeExpression> {
    parse_type_expression_with_rules(TypeExpressionParseRules::default())(input)
}

#[derive(Debug, Clone, Copy)]
pub struct TypeExpressionParseRules {
    // These rules exist to prevent infinite recursion (left recursion)
    pub allow_intersection: bool,
    pub allow_union: bool,
}

impl TypeExpressionParseRules {
    pub fn default() -> Self {
        Self {
            allow_intersection: true,
            allow_union: true,
        }
    }

    pub fn with_intersection(self, allow_intersection: bool) -> Self {
        Self {
            allow_intersection,
            ..self
        }
    }

    pub fn with_union(self, allow_union: bool) -> Self {
        Self {
            allow_union,
            ..self
        }
    }
}
