use lazy_static::lazy_static;

use crate::{
    ast::{Expression, Literal, Number, Operator, SingleOperation},
    core::{LocatedToken, Span, ParserResult},
    lexer::{Token, TokenKind},
    utils::{vec_alt, ws},
};

use super::{parse_expression, parse_expression_with_rule, ExpressionParseRules};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
struct OrderedOperator {
    token: TokenKind,
    operator: Operator,
    priority: u8,
    allow_repeat: bool,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq)]
struct OrderedSingleOperator<'a> {
    token: Token<'a>,
    operator: SingleOperation,
    priority: u8,
    side: Direction,
}

const OPERATORS: [OrderedOperator; 16] = [
    OrderedOperator {
        token: TokenKind::Range,
        operator: Operator::Range,
        priority: 0,
        allow_repeat: false,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::And,
        operator: Operator::And,
        priority: 1,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::Or,
        operator: Operator::Or,
        priority: 1,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::Equal,
        operator: Operator::Equals,
        priority: 2,
        allow_repeat: false,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::NotEqual,
        operator: Operator::NotEquals,
        priority: 2,
        allow_repeat: false,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::LessThan,
        operator: Operator::LessThan,
        priority: 2,
        allow_repeat: false,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::LessThanOrEqual,
        operator: Operator::LessThanOrEquals,
        priority: 2,
        allow_repeat: false,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::GreaterThan,
        operator: Operator::GreaterThan,
        priority: 2,
        allow_repeat: false,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::GreaterThanOrEqual,
        operator: Operator::GreaterThanOrEquals,
        priority: 2,
        allow_repeat: false,
        direction: Direction::Right,
    },
    //OrderedOperator {
    //    token: Token::NullCoalesce,
    //    operator: Operator::Coalesce,
    //    priority: 3,
    //    allow_repeat: true,
    //    direction: Direction::Right,
    //}, // This operator has a collision with the unwrap operator (?) so it is disabled for now
    OrderedOperator {
        token: TokenKind::Plus,
        operator: Operator::Add,
        priority: 4,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::Minus,
        operator: Operator::Subtract,
        priority: 4,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::Star,
        operator: Operator::Multiply,
        priority: 5,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::Divide,
        operator: Operator::Divide,
        priority: 5,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::Modulo,
        operator: Operator::Modulo,
        priority: 5,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        token: TokenKind::Power,
        operator: Operator::Power,
        priority: 6,
        allow_repeat: true,
        direction: Direction::Left,
    },
    OrderedOperator {
        token: TokenKind::Dot,
        operator: Operator::Access,
        priority: 10,
        allow_repeat: true,
        direction: Direction::Left,
    },
];

const SINGLE_OPERATORS: [OrderedSingleOperator; 6] = [
    OrderedSingleOperator {
        token: Token::Exclamation,
        operator: SingleOperation::Not,
        priority: 8,
        side: Direction::Left,
    },
    OrderedSingleOperator {
        token: Token::Minus,
        operator: SingleOperation::Negate,
        priority: 4,
        side: Direction::Left,
    },
    OrderedSingleOperator {
        token: Token::Plus,
        operator: SingleOperation::Positive,
        priority: 4,
        side: Direction::Left,
    },
    OrderedSingleOperator {
        token: Token::Question,
        operator: SingleOperation::ErrorUnwrap,
        priority: 9,
        side: Direction::Right,
    },
    OrderedSingleOperator {
        token: Token::Exclamation,
        operator: SingleOperation::Panic,
        priority: 9,
        side: Direction::Right,
    },
    OrderedSingleOperator {
        token: Token::Star,
        operator: SingleOperation::Spread,
        priority: 7,
        side: Direction::Left,
    },
];

lazy_static! {
    // Sort the operators by length, so that we can parse the longest operators first
    static ref OPERATORS_BY_LENGTH: Vec<&'static OrderedOperator> = {
        let mut operators: Vec<_> = OPERATORS.iter().collect();
        operators.sort_by(|a, b| b.token.len().cmp(&a.token.len()));
        operators
    };
}

#[derive(Debug, Clone, PartialEq)]
enum SingleOperatorData<'a> {
    Standalone(&'a OrderedSingleOperator<'a>),
    Call(Vec<Expression>),
    Get(Expression),
}

impl<'a> SingleOperatorData<'a> {
    fn priority(&self) -> u8 {
        match self {
            SingleOperatorData::Standalone(operator) => operator.priority,
            SingleOperatorData::Call(_) => 9,
            SingleOperatorData::Get(_) => 9,
        }
    }

    // fn side(&self) -> Direction {
    //     match self {
    //         SingleOperatorData::Standalone(operator) => operator.side,
    //         SingleOperatorData::Call(_) => Direction::Right,
    //         SingleOperatorData::Get(_) => Direction::Right,
    //     }
    // }
}

#[derive(Debug, Clone, PartialEq)]
struct Segment<'a> {
    expression: Expression,
    left_operations: Vec<SingleOperatorData<'a>>,
    right_operations: Vec<SingleOperatorData<'a>>,
}

impl<'a> Segment<'a> {
    fn new(
        expression: Expression,
        left_operations: Vec<SingleOperatorData>,
        right_operations: Vec<SingleOperatorData>,
    ) -> Self {
        Self {
            expression,
            left_operations,
            right_operations,
        }
    }

    fn from_expression(expression: Expression) -> Self {
        Self {
            expression,
            left_operations: Vec::new(),
            right_operations: Vec::new(),
        }
    }

    fn add_left_operation(&mut self, operation: SingleOperatorData) {
        // Add a single operation to the left side of the expression
        // Operations are added from the inside out, so the first operation
        // Example:
        //    push1 : -
        //    push2 : !
        //    output: !-value

        self.left_operations.push(operation);
    }
    fn add_right_operation(&mut self, operation: SingleOperatorData) {
        // Add a single operation to the right side of the expression
        // Operations are added from the inside out, so the first operation
        // Example:
        //    push1 : ?
        //    push2 : !
        //    output: value?!

        self.right_operations.push(operation);
    }

    fn peek_left_operation(&self) -> Option<SingleOperatorData> {
        // Peek the outermost left operation
        self.left_operations.last().cloned()
    }
    fn peek_right_operation(&self) -> Option<SingleOperatorData> {
        // Peek the outermost right operation
        self.right_operations.last().cloned()
    }

    fn remove_left_operation(&mut self) -> Option<SingleOperatorData> {
        // Remove the outermost left operation
        self.left_operations.pop()
    }
    fn remove_right_operation(&mut self) -> Option<SingleOperatorData> {
        // Remove the outermost right operation
        self.right_operations.pop()
    }

    fn move_left_operation(&mut self, other: &mut Self) {
        // Move the outermost left operation to another segment
        if let Some(operation) = self.remove_left_operation() {
            other.add_left_operation(operation);
        } else {
            panic!("No left operation to move");
        }
    }

    fn move_right_operation(&mut self, other: &mut Self) {
        // Move the outermost right operation to another segment
        if let Some(operation) = self.remove_right_operation() {
            other.add_right_operation(operation);
        } else {
            panic!("No right operation to move");
        }
    }

    fn has_operations(&self) -> bool {
        // Check if the segment has any operations
        !self.left_operations.is_empty() || !self.right_operations.is_empty()
    }

    fn to_expression(&self) -> Expression {
        // Sort the operations (left and right) by priority
        // Then, apply the operations to the expression

        let mut operations = self
            .left_operations
            .iter()
            .chain(self.right_operations.iter())
            .collect::<Vec<_>>();

        operations.sort_by(|a, b| a.priority().cmp(&b.priority()));

        let mut expression = self.expression.clone();

        for operation in operations {
            expression = match operation {
                SingleOperatorData::Standalone(single_operator) => {
                    Expression::SingleOperation(single_operator.operator, Box::new(expression))
                }
                SingleOperatorData::Call(arguments) => {
                    Expression::Call(Box::new(expression), arguments.clone())
                }
                SingleOperatorData::Get(index) => {
                    Expression::Get(Box::new(expression), Box::new(index.clone()))
                }
            }
        }

        expression
    }

    fn join_with(&self, operator: OrderedOperator, other: &Self) -> Self {
        // join segments, move operations if low priority
        // EXAMPLES
        // S1: -5       OP **   S2: 2       ->  S -(5 ** 2)
        // S1: !test    OP .    S2 field    ->  S !(test.field)
        // S1: -5       OP ??   S2: 2       ->  S (-5) ?? (2)
        // S1: -!5      OP **   S2: 6       ->  S -((!5) ** 6)
        // S1: 3        OP +    S2: 4       ->  S 3 + 4

        let mut new_segment =
            Segment::from_expression(Expression::Literal(Literal::Number(Number::Float(0.))));

        let mut left = self.clone();
        let mut right = other.clone();

        while let Some(left_operation) = left.peek_left_operation() {
            if left_operation.priority() < operator.priority {
                left.move_left_operation(&mut new_segment);
            } else {
                break;
            }
        }

        while let Some(right_operation) = right.peek_right_operation() {
            if right_operation.priority() < operator.priority {
                right.move_right_operation(&mut new_segment);
            } else {
                break;
            }
        }

        new_segment.expression = Expression::Operation(
            Box::new(left.to_expression()),
            operator.operator,
            Box::new(right.to_expression()),
        );

        new_segment
    }
}

fn parse_single_operator(side: Direction) -> impl Fn(Span) -> ParserResult<SingleOperatorData, String> {
    // Parse a single operator
    // Example: !, -, +, etc
    // This also includes function calls and array / object indexing (on the right side)

    move |input: Span| {
        let mut single_parser = vec_alt(
            SINGLE_OPERATORS
                .iter()
                .filter(|o| o.side == side)
                .map(|o| {
                    Box::new(map(tag(o.string), move |_| {
                        SingleOperatorData::Standalone(o)
                    }))
                })
                .collect::<Vec<_>>(),
        );

        let (input, operator) = match side {
            Direction::Left => single_parser(input),
            Direction::Right => alt((
                single_parser,
                map(
                    delimited(
                        tuple((char('('), ws)),
                        separated_list0(tuple((ws, char(','), ws)), parse_expression),
                        tuple((ws, char(')'))),
                    ),
                    |o| SingleOperatorData::Call(o),
                ),
                map(
                    delimited(
                        tuple((char('['), ws)),
                        parse_expression,
                        tuple((ws, char(']'))),
                    ),
                    |o| SingleOperatorData::Get(o),
                ),
            ))(input),
        }?;
        let (input, _) = ws(input)?;

        Ok((input, operator))
    }
}

fn parse_segment(rules: ExpressionParseRules) -> impl Fn(Span) -> IResult<Span, Segment> {
    move |input: Span| {
        let mut input = input;
        let mut left_side_operators = Vec::new();
        let left_side_single_operator = parse_single_operator(Direction::Left);
        while let Ok((new_input, operator)) = left_side_single_operator(input) {
            left_side_operators.push(operator);
            input = new_input;
        }
        let (input, expr) = parse_expression_with_rule(rules.with_operation(false))(input)?;
        let (mut input, _) = ws(input)?;
        let mut right_side_operators = Vec::new();
        let right_side_single_operator = parse_single_operator(Direction::Right);
        while let Ok((new_input, operator)) = right_side_single_operator(input) {
            right_side_operators.push(operator);
            input = new_input;
        }

        // reverse the order of the left side operators, so they are in the correct order (inside out)
        left_side_operators.reverse();

        let (input, _) = ws(input)?;

        let segment = Segment::new(expr, left_side_operators, right_side_operators);

        Ok((input, segment))
    }
}

fn parse_operator(input: Span) -> IResult<Span, OrderedOperator> {
    for operator in OPERATORS_BY_LENGTH.iter() {
        if let Ok((input, _)) = tag::<_, _, nom::error::Error<Span>>(operator.string)(input) {
            return Ok((input, (*operator).clone()));
        }
    }
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::IsNot,
    )))
}

fn parse_operator_and_segment(input: Span) -> IResult<Span, (OrderedOperator, Segment)> {
    let (input, _) = ws(input)?;
    let (input, operator) = parse_operator(input)?;
    let (input, _) = ws(input)?;
    // allow calls if operator has a higher priority than the call operator
    let call_priority = 9;
    let used_rules = match operator.priority {
        p if p < call_priority => ExpressionParseRules::default(),
        _ => ExpressionParseRules::default().with_call(false),
    };
    let (input, segment) = parse_segment(used_rules)(input)?;
    Ok((input, (operator, segment)))
}

pub fn parse_operation(rules: ExpressionParseRules) -> impl Fn(Span) -> IResult<Span, Expression> {
    move |input: Span| {
        let (input, mut left_side) = parse_segment(rules)(input)?;
        let (mut input, (mut operator, mut right_side)) =
            match parse_operator_and_segment(input.clone()) {
                Ok((input, (operator, right_side))) => Ok((input, (operator, right_side))),
                Err(_) => {
                    if left_side.has_operations() {
                        return Ok((input, left_side.to_expression()));
                    } else {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::IsNot,
                        )));
                    }
                }
            }?;

        while let Ok((input2, (next_operator, next_expression))) = parse_operator_and_segment(input)
        {
            if next_operator.priority > operator.priority
                || next_operator.direction == Direction::Left
            {
                // next operator has higher priority, or is left associative
                // join the next expression with the current one
                right_side = right_side.join_with(next_operator, &next_expression);
            } else {
                // next operator has lower priority, or is right associative
                // join the current expression with the next one
                left_side = left_side.join_with(operator, &right_side);
                operator = next_operator;
                right_side = next_expression;
            }
            input = input2;
        }

        let final_segment = left_side.join_with(operator, &right_side);

        Ok((input, final_segment.to_expression()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{expression::parse_expression, utils::new_span};
    use num::BigInt;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn order() {
        let tests = vec![
            (
                "-5 ** 2",
                parse_expression(new_span("-(5 ** 2)")).unwrap().1,
            ),
            (
                "!test.field",
                parse_expression(new_span("!(test.field)")).unwrap().1,
            ),
            (
                "-5 == 2",
                parse_expression(new_span("(-5) == (2)")).unwrap().1,
            ),
            (
                "-!5 ** 6",
                parse_expression(new_span("-((!5) ** 6)")).unwrap().1,
            ),
            (
                "5 ** 2 ** 3",
                parse_expression(new_span("5 ** (2 ** 3)")).unwrap().1,
            ),
            (
                "5 ** 2 * 3",
                parse_expression(new_span("(5 ** 2) * 3")).unwrap().1,
            ),
            (
                "5 * 2 ** 3",
                parse_expression(new_span("5 * (2 ** 3)")).unwrap().1,
            ),
            (
                "5 * 2 * 3",
                parse_expression(new_span("(5 * 2) * 3")).unwrap().1,
            ),
            (
                "5 + 2 * 3",
                parse_expression(new_span("5 + (2 * 3)")).unwrap().1,
            ),
            (
                "5 * 2 + 3",
                parse_expression(new_span("(5 * 2) + 3")).unwrap().1,
            ),
            (
                "5 + 2 + 3",
                parse_expression(new_span("(5 + 2) + 3")).unwrap().1,
            ),
            (
                "5 + 2 - 3",
                parse_expression(new_span("(5 + 2) - 3")).unwrap().1,
            ),
            (
                "3 + 4",
                Expression::Operation(
                    Box::new(Expression::Literal(Literal::Integer(BigInt::from(3)))),
                    Operator::Add,
                    Box::new(Expression::Literal(Literal::Integer(BigInt::from(4)))),
                ),
            ),
        ];

        for (input, expected) in tests {
            let (input, result) =
                parse_operation(ExpressionParseRules::default())(new_span(input)).unwrap();
            assert_eq!(input.fragment(), &"");
            assert_eq!(result, expected);
        }
    }
}