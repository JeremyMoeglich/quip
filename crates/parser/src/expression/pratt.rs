use fst::*;
use enumset::{enum_set_union, EnumSet};
use lexer::TokenKind;

use crate::{
    block::parse_block,
    destructure::parse_immutable_extract,
    utils::{opt, token_branch, ws0},
};
use parser_core::*;

use super::{
    array_expr::parse_array_expr,
    call_arguments::parse_call_arguments,
    closure_expr::parse_closure_expr,
    declaration_expr::parse_declaration_expr,
    identifier_expr::parse_variable_expr,
    if_expr::parse_if_expr,
    literal_expr::parse_literal_expr,
    loops_expr::{parse_for_expr, parse_loop_expr, parse_while_expr},
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct InfixOperator {
    token: TokenKind,
    operator: Operator,
    disallow_kinds: EnumSet<TokenKind>,
    left_binding: u8,
    right_binding: u8,
}

#[derive(Debug, Clone, PartialEq)]
struct UnaryOperator {
    token: TokenKind,
    operator: UnaryOperation,
    binding: u8,
}

const ORDERING_EQUALITY_SET: EnumSet<TokenKind> = enum_set_union!(
    TokenKind::Equal,
    TokenKind::NotEqual,
    TokenKind::LessThan,
    TokenKind::LessThanOrEqual,
    TokenKind::GreaterThan,
    TokenKind::GreaterThanOrEqual
);

const INFIX_OPERATORS: [InfixOperator; 23] = [
    InfixOperator {
        // Assignment operator (right associative)
        token: TokenKind::Assignment,
        operator: Operator::Assignment,
        disallow_kinds: EnumSet::empty(),
        left_binding: 2,
        right_binding: 0, // Right associative
    },
    InfixOperator {
        // Pipe (left associative)
        token: TokenKind::Pipe,
        operator: Operator::Pipe,
        disallow_kinds: EnumSet::empty(),
        left_binding: 4,
        right_binding: 3, // Left associative
    },
    InfixOperator {
        // Range operator (non-associative)
        token: TokenKind::Range,
        operator: Operator::Range,
        disallow_kinds: enum_set_union!(TokenKind::Range),
        left_binding: 6,
        right_binding: 6, // Non-associative
    },
    InfixOperator {
        // Logical AND (left associative)
        token: TokenKind::And,
        operator: Operator::And,
        disallow_kinds: EnumSet::empty(),
        left_binding: 7,
        right_binding: 8, // Left associative
    },
    InfixOperator {
        // Logical OR (left associative)
        token: TokenKind::Or,
        operator: Operator::Or,
        disallow_kinds: EnumSet::empty(),
        left_binding: 9,
        right_binding: 10, // Left associative
    },
    InfixOperator {
        // Equals (non-associative)
        token: TokenKind::Equal,
        operator: Operator::Equals,
        disallow_kinds: ORDERING_EQUALITY_SET,
        left_binding: 12,
        right_binding: 12, // Non-associative
    },
    InfixOperator {
        // Not Equals (non-associative)
        token: TokenKind::NotEqual,
        operator: Operator::NotEquals,
        disallow_kinds: ORDERING_EQUALITY_SET,
        left_binding: 12,
        right_binding: 12, // Non-associative
    },
    InfixOperator {
        // Less Than (non-associative)
        token: TokenKind::LessThan,
        operator: Operator::LessThan,
        disallow_kinds: ORDERING_EQUALITY_SET,
        left_binding: 12,
        right_binding: 12, // Non-associative
    },
    InfixOperator {
        // Less Than or Equal (non-associative)
        token: TokenKind::LessThanOrEqual,
        operator: Operator::LessThanOrEquals,
        disallow_kinds: ORDERING_EQUALITY_SET,
        left_binding: 12,
        right_binding: 12, // Non-associative
    },
    InfixOperator {
        // Greater Than (non-associative)
        token: TokenKind::GreaterThan,
        operator: Operator::GreaterThan,
        disallow_kinds: ORDERING_EQUALITY_SET,
        left_binding: 12,
        right_binding: 12, // Non-associative
    },
    InfixOperator {
        // Greater Than or Equal (non-associative)
        token: TokenKind::GreaterThanOrEqual,
        operator: Operator::GreaterThanOrEquals,
        disallow_kinds: ORDERING_EQUALITY_SET,
        left_binding: 12,
        right_binding: 12, // Non-associative
    },
    InfixOperator {
        // Union (left associative)
        token: TokenKind::VerticalBar,
        operator: Operator::Union,
        disallow_kinds: EnumSet::empty(),
        left_binding: 15,
        right_binding: 16, // Left associative
    },
    InfixOperator {
        // Intersection (left associative)
        token: TokenKind::Ampersand,
        operator: Operator::Intersection,
        disallow_kinds: EnumSet::empty(),
        left_binding: 15,
        right_binding: 16, // Left associative
    },
    InfixOperator {
        // Exclusive OR (left associative)
        token: TokenKind::Caret,
        operator: Operator::ExclusiveOr,
        disallow_kinds: EnumSet::empty(),
        left_binding: 15,
        right_binding: 16, // Left associative
    },
    InfixOperator {
        // Addition (left associative)
        token: TokenKind::Plus,
        operator: Operator::Add,
        disallow_kinds: EnumSet::empty(),
        left_binding: 17,
        right_binding: 18, // Left associative
    },
    InfixOperator {
        // Subtraction (left associative)
        token: TokenKind::Minus,
        operator: Operator::Subtract,
        disallow_kinds: EnumSet::empty(),
        left_binding: 17,
        right_binding: 18, // Left associative
    },
    InfixOperator {
        // Wrapping Addition (left associative)
        token: TokenKind::PlusPercent,
        operator: Operator::WrappingAdd,
        disallow_kinds: EnumSet::empty(),
        left_binding: 17,
        right_binding: 18, // Left associative
    },
    InfixOperator {
        // Wrapping Subtraction (left associative)
        token: TokenKind::MinusPercent,
        operator: Operator::WrappingSubtract,
        disallow_kinds: EnumSet::empty(),
        left_binding: 17,
        right_binding: 18, // Left associative
    },
    InfixOperator {
        // Multiplication (left associative)
        token: TokenKind::Star,
        operator: Operator::Multiply,
        disallow_kinds: EnumSet::empty(),
        left_binding: 19,
        right_binding: 20, // Left associative
    },
    InfixOperator {
        // Division (left associative)
        token: TokenKind::Divide,
        operator: Operator::Divide,
        disallow_kinds: EnumSet::empty(),
        left_binding: 19,
        right_binding: 20, // Left associative
    },
    InfixOperator {
        // Modulo (left associative)
        token: TokenKind::Modulo,
        operator: Operator::Modulo,
        disallow_kinds: EnumSet::empty(),
        left_binding: 19,
        right_binding: 20, // Left associative
    },
    InfixOperator {
        // Wrapping Multiplication (left associative)
        token: TokenKind::StarPercent,
        operator: Operator::WrappingMultiply,
        disallow_kinds: EnumSet::empty(),
        left_binding: 19,
        right_binding: 20, // Left associative
    },
    InfixOperator {
        // Power (right associative)
        token: TokenKind::Power,
        operator: Operator::Power,
        disallow_kinds: EnumSet::empty(),
        left_binding: 22,
        right_binding: 21, // Right associative
    },
];

const PREFIX_OPERATORS: [UnaryOperator; 4] = [
    UnaryOperator {
        // Logical NOT
        token: TokenKind::Exclamation,
        operator: UnaryOperation::Not,
        binding: 23,
    },
    UnaryOperator {
        // Negate
        token: TokenKind::Minus,
        operator: UnaryOperation::Negate,
        binding: 18,
    },
    UnaryOperator {
        // Positive (No operation)
        token: TokenKind::Plus,
        operator: UnaryOperation::Positive,
        binding: 18,
    },
    UnaryOperator {
        // Spread
        token: TokenKind::Star,
        operator: UnaryOperation::Spread,
        binding: 26,
    },
];

const POSTFIX_OPERATORS: [UnaryOperator; 2] = [
    UnaryOperator {
        // Error unwrap
        token: TokenKind::Question,
        operator: UnaryOperation::ErrorUnwrap,
        binding: 28,
    },
    UnaryOperator {
        // Inline
        token: TokenKind::Exclamation,
        operator: UnaryOperation::Inline,
        binding: 28,
    },
];

#[derive(Debug, Clone, PartialEq)]
struct PrattUnary {
    operation: UnaryOperation,
    binding: u8,
}

fn parse_prefix<'a>(input: Span<'a>) -> ParserResult<'a, PrattUnary> {
    let basic_prefix = token_branch(&PREFIX_OPERATORS, |op| op.token).map(|operator| PrattUnary {
        operation: operator.operator.clone(),
        binding: operator.binding,
    });
    let reference = |input| {
        let (input, _) = parse_ampersand(input)?;
        let (input, mutable) = opt((parse_mut, ws0).tuple()).map(|v| v.is_some())(input);
        Ok((
            input,
            PrattUnary {
                binding: 0,
                operation: UnaryOperation::Reference { mutable },
            },
        ))
    };
    (basic_prefix, reference).alt()(input)
}

fn parse_postfix<'a>(input: Span<'a>) -> ParserResult<'a, PrattUnary> {
    let basic_postfix =
        token_branch(&POSTFIX_OPERATORS, |op| op.token).map(|operator| PrattUnary {
            operation: operator.operator.clone(),
            binding: operator.binding,
        });
    let extract = |input| {
        let (input, _) = parse_dot(input)?;
        let (input, _) = ws0(input);
        let (input, extract) = parse_immutable_extract(input)?;
        Ok((
            input,
            PrattUnary {
                binding: 0,
                operation: UnaryOperation::Extract { extract },
            },
        ))
    };
    let function_call = |input| {
        let (input, arguments) = parse_call_arguments(input)?;
        Ok((
            input,
            PrattUnary {
                binding: 0,
                operation: UnaryOperation::Call { arguments },
            },
        ))
    };
    let get_property = |input| {
        let (input, _) = parse_left_bracket(input)?;
        let (input, _) = ws0(input);
        let (input, property) = parse_expression(input)?;
        let (input, _) = ws0(input);
        let (input, _) = parse_right_bracket(input)?;
        Ok((
            input,
            PrattUnary {
                binding: 0,
                operation: UnaryOperation::Get {
                    property: Box::new(property),
                },
            },
        ))
    };
    (basic_postfix, extract, function_call, get_property).alt()(input)
}

fn parse_infix<'a>(input: Span<'a>) -> ParserResult<'a, InfixOperator> {
    let branch = token_branch(&INFIX_OPERATORS, |op| op.token);
    branch(input).map(|(input, operator)| (input, operator.clone()))
}

fn parse_primary<'a>(input: Span<'a>) -> ParserResult<'a, Expression> {
    // None of these will ever start with an expression
    (
        delimited(
            (parse_left_paren, ws0).tuple(),
            parse_expression,
            (ws0, parse_right_paren).tuple(),
        ),
        parse_variable_expr,
        parse_literal_expr,
        parse_array_expr,
        parse_declaration_expr,
        parse_closure_expr,
        parse_if_expr,
        parse_while_expr,
        parse_loop_expr,
        parse_for_expr,
        parse_block.map(|code| Expression::Block {
            block: code,
            environment: None,
        }),
    )
        .alt()(input)
}

enum PrattOperator {
    Postfix(PrattUnary),
    Infix(InfixOperator),
}

impl PrattOperator {
    fn left_binding(&self) -> u8 {
        match self {
            PrattOperator::Postfix(op) => op.binding,
            PrattOperator::Infix(op) => op.left_binding,
        }
    }
}

fn parse_pratt_operator<'a>(input: Span<'a>) -> ParserResult<'a, PrattOperator> {
    (
        parse_infix.map(|op| PrattOperator::Infix(op)),
        parse_postfix.map(|op| PrattOperator::Postfix(op)),
    )
        .alt()(input)
}

pub fn parse_expression<'a>(input: Span<'a>) -> ParserResult<'a, Expression> {
    let (input, (expr, pratt_operator)) = parse_expression_pratt(input, 0)?;
    debug_assert!(pratt_operator.is_none());
    Ok((input, expr))
}

// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
// This function implements Pratt parsing, a top-down operator precedence parser
// for handling expressions with operators of varying precedence levels.
// It takes the input span and the minimum left binding power, returning the parsed
// expression and possibly the next Pratt operator.
fn parse_expression_pratt<'a>(
    input: Span<'a>,
    min_left_binding: u8,
) -> ParserResult<'a, (Expression, Option<PrattOperator>)> {
    // Begin by attempting to parse a prefix operator (e.g., unary minus, logical NOT).
    // If a prefix operator is found, parse the corresponding right operand recursively
    // with the prefix's binding power as the new minimum binding power.
    let (mut left_input, (mut left, mut pratt_operator)) = {
        match parse_prefix(input) {
            // A prefix operator is found
            Ok((input, prefix)) => {
                // Consume any whitespace following the prefix.
                let (input, _) = ws0(input);

                // Recursively parse the right-hand side of the prefix operation.
                // The prefix operator's binding power dictates the new minimum binding power.
                let (input, (right, next_pratt_operator)) =
                    parse_expression_pratt(input, prefix.binding)?;

                // Construct the expression for this prefix operation.
                let expr = Expression::SingleOperation {
                    operation: prefix.operation,
                    operand: Box::new(right),
                };

                // next_pratt_operator is present IF the called parse_pratt_operator
                // successfully parsed an operator, but had a lower binding power than
                // the current operator.
                match next_pratt_operator {
                    Some(next_pratt_operator) => (input, (expr, next_pratt_operator)),
                    // If no more operators are found, return the current expression.
                    None => return Ok((input, (expr, None))),
                }
            }
            Err(e1) => {
                // If no prefix is found, parse the primary expression (like literals, variables, etc.).
                let (pre_ws0_input, expr) = parse_primary(input).map_err(|e2| e1.accumulate(e2))?;

                // Consume any whitespace after the primary expression.
                let (input, _) = ws0(pre_ws0_input);

                // Check if there is a Pratt operator (infix or postfix) immediately following.
                let (input, next_pratt_operator) = match parse_pratt_operator(input) {
                    Ok((input, operator)) => (input, operator),
                    // If no operator is found, return the parsed primary expression.
                    Err(_) => return Ok((pre_ws0_input, (expr, None))),
                };

                // Set up the left expression and current operator for further processing in the loop.
                (input, (expr, next_pratt_operator))
            }
        }
    };

    // This loop processes operators to build the correct precedence structure for expressions.
    loop {
        // If the current operator's binding power is less than the minimum binding power
        // required, the loop breaks, ending the recursive parsing.
        if pratt_operator.left_binding() < min_left_binding {
            return Ok((left_input, (left, Some(pratt_operator))));
        }

        // Consume any whitespace after the left expression or operator.
        let (input, _) = ws0(left_input);

        // Match on the type of operator: infix or postfix.
        match pratt_operator {
            // Handle infix operators like +, -, *, etc.
            PrattOperator::Infix(operator) => {
                // Recursively parse the right-hand side of the infix operation.
                // The operator's right binding power dictates the new minimum binding power.
                let (input, (right, next_pratt_operator)) =
                    parse_expression_pratt(input, operator.right_binding)?;

                // Construct the expression for this infix operation.
                left = Expression::Operation {
                    left: Box::new(left),
                    operator: operator.operator,
                    right: Box::new(right),
                };

                // Update the current operator or finish if there are no more operators.
                match next_pratt_operator {
                    Some(operator) => {
                        pratt_operator = operator;
                        left_input = input;
                    }
                    None => return Ok((input, (left, None))),
                }
            }
            // Handle postfix operators like ! (error unwrap), [] (property access), etc.
            PrattOperator::Postfix(operator) => {
                // Construct the expression for this postfix operation.
                left = Expression::SingleOperation {
                    operation: operator.operation,
                    operand: Box::new(left),
                };

                // Check if there's another Pratt operator to process.
                match parse_pratt_operator(input) {
                    Ok((input, operator)) => {
                        pratt_operator = operator;
                        left_input = input;
                    }
                    // If no more operators, return the constructed expression.
                    Err(_) => return Ok((left_input, (left, None))),
                };
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::ParseString;

    use super::*;

    #[test]
    fn test_single_number() {
        let input = "5";
        let expected = Expression::Literal {
            value: Literal::Number("5".to_string()),
        };
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_not_field_access() {
        let input = "test.field";
        let expected = Expression::SingleOperation {
            operation: UnaryOperation::Extract {
                extract: ImmutableExtract::DirectProperty(Box::new(ImmutableDestructureProperty {
                    property_name: String::from("field"),
                    extract: None,
                    alias: None,
                })),
            },
            operand: Box::new(Expression::Variable {
                identifier: String::from("test"),
            }),
        };
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_negation_equality() {
        let input = "-5 == 2";
        let expected = parse_expression.parse_string("(-5) == (2)").unwrap();
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_not_exponentiation() {
        let input = "-!5 ** 6";
        let expected = parse_expression.parse_string("-((!5) ** 6)").unwrap();
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_double_exponentiation() {
        let input = "5 ** 2 ** 3";
        let expected = parse_expression.parse_string("5 ** (2 ** 3)").unwrap();
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_exponentiation_multiplication() {
        let input = "5 ** 2 * 3";
        let expected = parse_expression.parse_string("(5 ** 2) * 3").unwrap();
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiplication_exponentiation() {
        let input = "5 * 2 ** 3";
        let expected = parse_expression.parse_string("5 * (2 ** 3)").unwrap();
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_double_multiplication() {
        let input = "5 * 2 * 3";
        let expected = parse_expression.parse_string("(5 * 2) * 3").unwrap();
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_addition_multiplication() {
        let input = "5 + 2 * 3";
        let expected = parse_expression.parse_string("5 + (2 * 3)").unwrap();
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiplication_addition() {
        let input = "5 * 2 + 3";
        let expected = parse_expression.parse_string("(5 * 2) + 3").unwrap();
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_double_addition() {
        let input = "5 + 2 + 3";
        let expected = parse_expression.parse_string("(5 + 2) + 3").unwrap();
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_addition_subtraction() {
        let input = "5 + 2 - 3";
        let expected = parse_expression.parse_string("(5 + 2) - 3").unwrap();
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_addition() {
        let input = "3 + 4";
        let expected = Expression::Operation {
            left: Box::new(Expression::Literal {
                value: Literal::Number("3".to_string()),
            }),
            operator: Operator::Add,
            right: Box::new(Expression::Literal {
                value: Literal::Number("4".to_string()),
            }),
        };
        let result = parse_expression.parse_string(input).unwrap();
        assert_eq!(result, expected);
    }
}
