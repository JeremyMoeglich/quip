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
    utils::{acond, ws0},
};

pub fn parse_expression<'a>(input: &Span<'a>) -> ParserResult<'a, Expression> {
    parse_expression_with_rule(ExpressionParseRules::default())(input)
}

pub fn parse_expression_with_rule(
    rules: ExpressionParseRules,
) -> impl for<'a> Fn(&Span<'a>) -> ParserResult<'a, Expression> {
    move |input: &Span| {
        let (input, expression) = (
            acond(rules.allow_operation, parse_operation(rules.clone())),
            parse_list,
            delimited(
                (token_parser!(nodata LeftParen), ws0).tuple(),
                parse_expression,
                (ws0, token_parser!(nodata RightParen)).tuple(),
            ),
            parse_object,
            delimited(
                (token_parser!(nodata LeftBrace), ws0).tuple(),
                parse_code,
                (ws0, token_parser!(nodata RightBrace)).tuple(),
            )
            .map(|code| Expression::Block(code)),
            parse_literal.map(|literal| Expression::Literal(literal)),
            parse_variable,
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
    use ast::Operator;
    use pretty_assertions::assert_eq;

    use crate::utils::static_span;

    use super::*;

    #[test]
    fn test_parse_expr() {
        let tests = vec![
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
            assert_eq!(result, expected);
        }
    }
}
