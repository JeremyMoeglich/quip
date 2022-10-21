mod after_single_operation;
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

use self::{
    after_single_operation::parse_after_single_operation,
    literal::parse_literal,
    operation::parse_operation, variable::parse_variable,
};

use super::utils::ws;

pub fn parse_expression(input: Span) -> IResult<Span, crate::ast::Expression> {
    parse_expression_with_rule(ExpressionParseRules::new())(input)
}

pub fn parse_expression_with_rule(
    rules: ExpressionParseRules,
) -> impl Fn(Span) -> IResult<Span, ast::Expression> {
    move |input: Span| {
        let (input, expression) = alt((
            |input2: _| match rules.operation {
                true => parse_operation(rules)(input2),
                false => Err(nom::Err::Error(nom::error::Error::new(
                    input2,
                    nom::error::ErrorKind::Alt,
                ))),
            },
            |input2: _| match rules.after_single_operation {
                true => parse_after_single_operation(rules)(input2),
                false => Err(nom::Err::Error(nom::error::Error::new(
                    input2,
                    nom::error::ErrorKind::Alt,
                ))),
            },
            |input2: _| match rules.call {
                true => call::parse_call(rules)(input2),
                false => Err(nom::Err::Error(nom::error::Error::new(
                    input2,
                    nom::error::ErrorKind::Alt,
                ))),
            },
            |input2: _| {
                delimited(
                    tuple((char('('), ws)),
                    parse_expression,
                    tuple((ws, char(')'))),
                )(input2)
            },
            map(parse_literal, |literal| ast::Expression::Literal(literal)),
            parse_variable,
        ))(input)?;
        Ok((input, expression))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExpressionParseRules {
    // These rules exist to prevent infinite recursion (left recursion)
    pub operation: bool,
    pub call: bool,
    pub after_single_operation: bool,
}

impl ExpressionParseRules {
    pub fn new() -> Self {
        Self {
            operation: true,
            call: true,
            after_single_operation: true,
        }
    }

    pub fn with_operation(mut self, operation: bool) -> Self {
        self.operation = operation;
        self
    }

    pub fn with_call(mut self, call: bool) -> Self {
        self.call = call;
        self
    }

    pub fn with_after_single_operation(mut self, after_single_operation: bool) -> Self {
        self.after_single_operation = after_single_operation;
        self
    }
}
