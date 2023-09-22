mod list;
mod literal;
mod object;
mod operation;
mod stringify;
mod variable;

use ast::Expression;
use parser_core::*;

use self::{
    list::parse_list, literal::parse_literal, object::parse_object, operation::parse_operation,
    variable::parse_variable,
};

use super::{
    parse_code,
    utils::{acond, ws0, AcondError},
};

use thiserror::Error;

use parser_core::*;

pub fn parse_expression<'a>(input: &Span<'a>) -> ParserResult<'a, Expression, TokenParserError> {
    parse_expression_with_rule(ExpressionParseRules::default())(input)
}

pub fn parse_expression_with_rule(
    rules: ExpressionParseRules,
) -> impl for<'a> Fn(&Span<'a>) -> ParserResult<'a, Expression, TokenParserError> {
    move |input: &Span| {
        let (input, expression) = (
            acond(rules.allow_operation, parse_operation(rules.clone())).map_err(|e| match e {
                AcondError::ParserError(e) => e,
                AcondError::ParserInactive => TokenParserSubError::WrongTokenKind.into(),
            }),
            parse_list.map_err(|e| e.into()),
            delimited(
                (token_parser!(nodata LeftParen), ws0).tuple(),
                parse_expression,
                (ws0, token_parser!(nodata RightParen)).tuple(),
            ),
            parse_object.map_err(|e| e.into()),
            delimited(
                (token_parser!(nodata LeftBrace), ws0).tuple(),
                parse_code,
                (ws0, token_parser!(nodata RightBrace)).tuple(),
            )
            .map(|code| Expression::Block(code)),
            parse_literal
                .map(|literal| Expression::Literal(literal))
                .map_err(|_| ()),
            parse_variable.map_err(|_| ()),
        )
            .alt()(input)?;
        Ok((input, expression))
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
    use ast::{Literal, Operator};
    use pretty_assertions::assert_eq;

    use crate::utils::static_span;

    use super::*;

    #[test]
    fn test_parse_expr() {
        let tests = vec![
            (
                r#""test""#,
                Expression::FancyString(vec![Literal::String("test".to_string())]),
            ),
            (
                "obj!.field(5, 2)",
                parse_expression(&static_span("((obj!).field)(5, 2)"))
                    .unwrap()
                    .1,
            ),
            (
                r#"("5".to_int() + 5)"#,
                parse_expression(&static_span(r#"((("5").to_int)() + 5)"#))
                    .unwrap()
                    .1,
            ),
            (
                r#""5".to_int()"#,
                Expression::Call(
                    Box::new(Expression::Operation(
                        Box::new(parse_expression(&static_span(r#""5""#)).unwrap().1),
                        Operator::Access,
                        Box::new(Expression::Variable("to_int".to_string())),
                    )),
                    vec![],
                ),
            ),
        ];

        for (input, expected) in tests {
            let (input, result) = parse_expression(&static_span(input)).unwrap();
            assert_eq!(input.fragment(), &"");
            assert_eq!(result, expected);
        }
    }
}
