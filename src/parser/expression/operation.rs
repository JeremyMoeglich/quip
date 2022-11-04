use lazy_static::lazy_static;
use nom::{bytes::complete::tag, combinator::map, IResult};

use crate::{
    ast::{Expression, Literal, Operator, SingleOperation},
    parser::utils::{vec_alt, ws, Span},
};

use super::{parse_expression_with_rule, ExpressionParseRules};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrderedOperator<'a> {
    string: &'a str,
    operator: Operator,
    priority: u8,
    allow_repeat: bool,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrderedSingleOperator<'a> {
    string: &'a str,
    operator: SingleOperation,
    priority: u8,
    side: Direction,
}

static OPERATORS: [OrderedOperator; 17] = [
    OrderedOperator {
        string: "..",
        operator: Operator::Range,
        priority: 0,
        allow_repeat: false,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: "&&",
        operator: Operator::And,
        priority: 1,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: "||",
        operator: Operator::Or,
        priority: 1,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: "==",
        operator: Operator::Equals,
        priority: 2,
        allow_repeat: false,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: "!=",
        operator: Operator::NotEquals,
        priority: 2,
        allow_repeat: false,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: "<",
        operator: Operator::LessThan,
        priority: 2,
        allow_repeat: false,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: "<=",
        operator: Operator::LessThanOrEquals,
        priority: 2,
        allow_repeat: false,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: ">",
        operator: Operator::GreaterThan,
        priority: 2,
        allow_repeat: false,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: ">=",
        operator: Operator::GreaterThanOrEquals,
        priority: 2,
        allow_repeat: false,
        direction: Direction::Right,
    },
    //OrderedOperator {
    //    string: "??",
    //    operator: Operator::Coalesce,
    //    priority: 3,
    //    allow_repeat: true,
    //    direction: Direction::Right,
    //}, // This operator has a collision with the unwrap operator (?) so it is disabled for now
    OrderedOperator {
        string: "+",
        operator: Operator::Add,
        priority: 4,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: "-",
        operator: Operator::Subtract,
        priority: 4,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: "*",
        operator: Operator::Multiply,
        priority: 5,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: "//",
        operator: Operator::IntDivide,
        priority: 5,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: "/",
        operator: Operator::Divide,
        priority: 5,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: "%",
        operator: Operator::Modulo,
        priority: 5,
        allow_repeat: true,
        direction: Direction::Right,
    },
    OrderedOperator {
        string: "**",
        operator: Operator::Power,
        priority: 6,
        allow_repeat: true,
        direction: Direction::Left,
    },
    OrderedOperator {
        string: ".",
        operator: Operator::Access,
        priority: 10,
        allow_repeat: true,
        direction: Direction::Left,
    },
];

static SINGLE_OPERATORS: [OrderedSingleOperator; 6] = [
    OrderedSingleOperator {
        string: "!",
        operator: SingleOperation::Not,
        priority: 8,
        side: Direction::Left,
    },
    OrderedSingleOperator {
        string: "-",
        operator: SingleOperation::Negate,
        priority: 4,
        side: Direction::Left,
    },
    OrderedSingleOperator {
        string: "+",
        operator: SingleOperation::Positive,
        priority: 4,
        side: Direction::Left,
    },
    OrderedSingleOperator {
        string: "?",
        operator: SingleOperation::ErrorUnwrap,
        priority: 9,
        side: Direction::Right,
    },
    OrderedSingleOperator {
        string: "!",
        operator: SingleOperation::Panic,
        priority: 9,
        side: Direction::Right,
    },
    OrderedSingleOperator {
        string: "...",
        operator: SingleOperation::Spread,
        priority: 7,
        side: Direction::Left,
    },
];

lazy_static! {
    // Sort the operators by length, so that we can parse the longest operators first
    static ref OPERATORS_BY_LENGTH: Vec<&'static OrderedOperator<'static>> = {
        let mut operators: Vec<_> = OPERATORS.iter().collect();
        operators.sort_by(|a, b| b.string.len().cmp(&a.string.len()));
        operators
    };
}

#[derive(Debug, Clone, PartialEq)]
struct Segment {
    expression: Expression,
    left_operations: Vec<&'static OrderedSingleOperator<'static>>,
    right_operations: Vec<&'static OrderedSingleOperator<'static>>,
}

impl Segment {
    fn new(
        expression: Expression,
        left_operations: Vec<&'static OrderedSingleOperator<'static>>,
        right_operations: Vec<&'static OrderedSingleOperator<'static>>,
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

    fn add_left_operation(&mut self, operation: &'static OrderedSingleOperator<'static>) {
        // Add a single operation to the left side of the expression
        // Operations are added from the inside out, so the first operation
        // Example:
        //    push1 : -
        //    push2 : !
        //    output: !-value

        self.left_operations.push(operation);
    }
    fn add_right_operation(&mut self, operation: &'static OrderedSingleOperator<'static>) {
        // Add a single operation to the right side of the expression
        // Operations are added from the inside out, so the first operation
        // Example:
        //    push1 : ?
        //    push2 : !
        //    output: value?!

        self.right_operations.push(operation);
    }

    fn peek_left_operation(&self) -> Option<&'static OrderedSingleOperator<'static>> {
        // Peek the outermost left operation
        self.left_operations.last().copied()
    }
    fn peek_right_operation(&self) -> Option<&'static OrderedSingleOperator<'static>> {
        // Peek the outermost right operation
        self.right_operations.last().copied()
    }

    fn remove_left_operation(&mut self) -> Option<&'static OrderedSingleOperator<'static>> {
        // Remove the outermost left operation
        self.left_operations.pop()
    }
    fn remove_right_operation(&mut self) -> Option<&'static OrderedSingleOperator<'static>> {
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

        operations.sort_by(|a, b| a.priority.cmp(&b.priority));
        let operations = operations.iter().map(|o| o.operator).collect::<Vec<_>>();

        let mut expression = self.expression.clone();

        for operation in operations {
            expression = Expression::SingleOperation(operation, Box::new(expression))
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

        let mut new_segment = Segment::from_expression(Expression::Literal(Literal::None));

        let mut left = self.clone();
        let mut right = other.clone();

        while let Some(left_operation) = left.peek_left_operation() {
            if left_operation.priority < operator.priority {
                left.move_left_operation(&mut new_segment);
            } else {
                break;
            }
        }

        while let Some(right_operation) = right.peek_right_operation() {
            if right_operation.priority < operator.priority {
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

fn parse_single_operator(
    side: Direction,
) -> impl Fn(Span) -> IResult<Span, &'static OrderedSingleOperator<'static>> {
    // Parse a single operator
    // Example: !, -, +, etc

    move |input: Span| {
        let (input, operator) = vec_alt(
            SINGLE_OPERATORS
                .iter()
                .filter(|o| o.side == side)
                .map(|o| Box::new(map(tag(o.string), move |_| o)))
                .collect::<Vec<_>>(),
        )(input)?;
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
    let input_copy = input.clone();
    let (input, _) = ws(input)?;
    let (input, operator) = parse_operator(input)?;
    let (input, _) = ws(input)?;
    // allow calls if operator has a higher priority than the call operator
    let call_priority = 9;
    let used_rules = match operator.priority {
        p if p > call_priority => ExpressionParseRules::default(),
        _ => ExpressionParseRules::default().with_call(false),
    };
    let (input, segment) = parse_segment(used_rules)(input)?;
    println!(
        "{:?} -> {:?}, rem: {}",
        input_copy.fragment(),
        segment,
        input.fragment()
    );
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
    use crate::parser::{expression::parse_expression, utils::new_span};
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
