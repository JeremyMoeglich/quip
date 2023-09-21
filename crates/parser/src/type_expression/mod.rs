mod array;
mod object;

use ast::TypeExpression;
use parser_core::*;

use crate::utils::ws_delimited;

use self::{array::parse_type_array, object::parse_type_object};
use super::{identifier::parse_identifier, utils::acond};

pub fn parse_type_expression_with_rules<'a>(
    rules: TypeExpressionParseRules,
) -> impl Fn(&Span<'a>) -> ParserResult<'a, TypeExpression, ()> {
    move |input| {
        (
            // parse_type_array.map(|types| TypeExpression::Tuple(types)),
            parse_type_object.map_err(|_| ()),
            acond(
                rules.allow_union,
                separated_list1(
                    ws_delimited(token_parser!(nodata VerticalBar)).map_err(|_: TokenParserError| ()),
                    parse_type_expression_with_rules(rules.with_union(false)),
                )
                .map(|expressions| match expressions.len() {
                    0 => unreachable!(),
                    1 => expressions.into_iter().next().unwrap(),
                    _ => TypeExpression::Union(expressions),
                }),
            )
            .map_err(|_| ()),
            acond(
                rules.allow_intersection,
                separated_list1(
                    ws_delimited(token_parser!(nodata Ampersand)).map_err(|_: TokenParserError| ()),
                    parse_type_expression_with_rules(rules.with_intersection(false)),
                )
                .map(|expressions| TypeExpression::Intersection(expressions)),
            )
            .map_err(|_| ()),
            parse_identifier
                .map(|identifier| TypeExpression::Variable(identifier))
                .map_err(|_| ()),
        )
            .alt()(input)
    }
}

pub fn parse_type_expression<'a>(
    input: &Span<'a>,
) -> ParserResult<'a, TypeExpression, TokenParserError> {
    parse_type_expression_with_rules(TypeExpressionParseRules::default())(input)
}

#[derive(Debug, Clone, Copy)]
pub struct TypeExpressionParseRules {
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
