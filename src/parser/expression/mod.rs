mod call;
mod literal;
mod operation;
mod variable;

use nom::{
    branch::alt,
    character::complete::char,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{ast, parser::utils::Span};

use self::{literal::parse_literal, operation::parse_operation, variable::parse_variable};

use super::utils::ws;

pub fn parse_expression(input: Span) -> IResult<Span, crate::ast::Expression> {
    parse_expression_with_rule(ExpressionParseRules::default())(input)
}

pub fn parse_expression_with_rule(
    rules: ExpressionParseRules,
) -> impl Fn(Span) -> IResult<Span, ast::Expression> {
    move |input: Span| {
        let (input, expression) = alt((
            |input2: _| match rules.allow_operation {
                true => parse_operation(rules.clone())(input2),
                false => Err(nom::Err::Error(nom::error::Error::new(
                    input2,
                    nom::error::ErrorKind::Alt,
                ))),
            },
            delimited(
                tuple((char('('), ws)),
                parse_expression,
                tuple((ws, char(')'))),
            ),
            map(parse_literal, |literal| ast::Expression::Literal(literal)),
            parse_variable,
        ))(input)?;
        match rules.allow_call {
            true => call::parse_call(expression)(input),
            false => Ok((input, expression)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExpressionParseRules {
    // These rules exist to prevent infinite recursion (left recursion)
    pub allow_operation: bool,
    pub allow_call: bool,
}

impl ExpressionParseRules {
    pub fn default() -> Self {
        Self {
            allow_operation: true,
            allow_call: true,
        }
    }

    pub fn with_operation(self, allow_operation: bool) -> Self {
        Self {
            allow_operation,
            ..self
        }
    }

    pub fn with_call(self, allow_call: bool) -> Self {
        Self { allow_call, ..self }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expression, FancyStringFragment, Literal},
        parser::{utils::new_span},
    };

    use super::*;

    #[test]
    fn test_parse_expr() {
        let tests = vec![
            (
                r#""test""#,
                Expression::Literal(Literal::String(vec![FancyStringFragment::LiteralString(
                    "test".to_string(),
                )])),
            ),
            (
                "obj!.field(5, 2)",
                parse_expression(new_span("((obj!).field)(5, 2)"))
                    .unwrap()
                    .1,
            ),
        ];

        for (input, expected) in tests {
            let (input, result) = parse_expression(new_span(input)).unwrap();
            assert_eq!(input.fragment(), &"");
            assert_eq!(result, expected);
        }
    }
}
