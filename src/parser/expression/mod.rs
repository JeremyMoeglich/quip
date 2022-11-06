mod list;
mod literal;
mod object;
mod operation;
mod variable;

use nom::{
    branch::alt,
    character::complete::char,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

use crate::parser::utils::Span;

use self::{
    list::parse_list, literal::parse_literal, object::parse_object, operation::parse_operation,
    variable::parse_variable,
};

use super::{
    ast::Expression,
    parse_code,
    utils::{acond, ws},
};

pub fn parse_expression(input: Span) -> IResult<Span, Expression> {
    parse_expression_with_rule(ExpressionParseRules::default())(input)
}

pub fn parse_expression_with_rule(
    rules: ExpressionParseRules,
) -> impl Fn(Span) -> IResult<Span, Expression> {
    move |input: Span| {
        let (input, expression) = alt((
            acond(rules.allow_operation, parse_operation(rules.clone())),
            parse_list,
            delimited(
                tuple((char('('), ws)),
                parse_expression,
                tuple((ws, char(')'))),
            ),
            parse_object,
            map(
                delimited(tuple((char('{'), ws)), parse_code, tuple((ws, char('}')))),
                |code| Expression::Block(code),
            ),
            map(parse_literal, |literal| Expression::Literal(literal)),
            parse_variable,
        ))(input)?;
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
    use crate::parser::{
        ast::{Expression, FancyStringFragment, Literal, Operator},
        utils::new_span,
    };
    use pretty_assertions::assert_eq;

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
            (
                r#"("5".to_int() + 5)"#,
                parse_expression(new_span(r#"((("5").to_int)() + 5)"#))
                    .unwrap()
                    .1,
            ),
            (
                r#""5".to_int()"#,
                Expression::Call(
                    Box::new(Expression::Operation(
                        Box::new(parse_expression(new_span(r#""5""#)).unwrap().1),
                        Operator::Access,
                        Box::new(Expression::Variable("to_int".to_string())),
                    )),
                    vec![],
                ),
            ),
        ];

        for (input, expected) in tests {
            let (input, result) = parse_expression(new_span(input)).unwrap();
            assert_eq!(input.fragment(), &"");
            assert_eq!(result, expected);
        }
    }
}
