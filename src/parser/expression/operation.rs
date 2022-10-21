use std::collections::HashMap;

use lazy_static::lazy_static;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{map, opt},
    multi::many1,
    IResult,
};
use num::BigInt;

use crate::{
    ast::{Expression, Literal, Operator, SingleOperation},
    parser::utils::{vec_alt, ws, Span},
};

use super::{parse_expression_with_rule, ExpressionParseRules};

#[derive(Debug)]

enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct OrderedOperator<'a> {
    string: &'a str,
    operator: Operator,
    priority: u8,
    allow_repeat: bool,
    direction: Direction,
}

static OPERATORS: [OrderedOperator; 18] = [
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
    OrderedOperator {
        string: "??",
        operator: Operator::Coalesce,
        priority: 3,
        allow_repeat: true,
        direction: Direction::Right,
    },
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
        priority: 7,
        allow_repeat: true,
        direction: Direction::Right,
    },
];

lazy_static! {
    static ref OPERATOR_MAP: HashMap<&'static str, &'static OrderedOperator<'static>> = {
        let mut map = HashMap::new();
        for operator in OPERATORS.iter() {
            map.insert(operator.string, operator);
        }
        map
    };
}

fn get_operator(string: &str) -> Option<&'static OrderedOperator> {
    OPERATOR_MAP.get(string).copied()
}

pub fn parse_operation(rules: ExpressionParseRules) -> impl Fn(Span) -> IResult<Span, Expression> {
    move |input: Span| {
        let parse_next = |get_operator: bool| {
            move |input: Span| {
                let (input, operator_opt) = {
                    if get_operator {
                        let (input, _) = ws(input)?;
                        map(
                            vec_alt(
                                OPERATORS
                                    .iter()
                                    .map(|op| map(tag(op.string), |_| op.operator))
                                    .collect(),
                            ),
                            Some,
                        )(input)?
                    } else {
                        (input, None)
                    }
                };
                let (input, _) = ws(input)?;
                let (input, before_operator_opt) = opt(alt((
                    map(tag("!"), |_| SingleOperation::Not),
                    map(tag("-"), |_| SingleOperation::Unary),
                    map(tag("..."), |_| SingleOperation::Spread),
                )))(input)?;
                let (input, _) = ws(input)?;
                let (input, expression) =
                    parse_expression_with_rule(rules.with_operation(false))(input)?;
                Ok((input, (operator_opt, before_operator_opt, expression)))
            }
        };
        let parse_next_with_operator = |input: Span| {
            let (input, (operator_opt, before_operator_opt, expression)) = parse_next(true)(input)?;
            if let Some(operator) = operator_opt {
                Ok((input, (operator, before_operator_opt, expression)))
            } else {
                Err(nom::Err::Error(nom::error::make_error(
                    input,
                    nom::error::ErrorKind::Tag,
                )))
            }
        };
        let parse_next_without_operator = |input: Span| {
            let (input, (operator_opt, before_operator_opt, expression)) =
                parse_next(false)(input)?;
            if operator_opt.is_none() {
                Ok((input, (before_operator_opt, expression)))
            } else {
                Err(nom::Err::Error(nom::error::make_error(
                    input,
                    nom::error::ErrorKind::Tag,
                )))
            }
        };

        let (input, lst) = {
            let mut lst = vec![];
            let input = input;
            let (input, (before_operator_opt, expression)) = parse_next_without_operator(input)?;
            if let Some(before_operator) = before_operator_opt {
                match before_operator {
                    SingleOperation::Unary => {
                        lst.push((
                            get_operator("+").unwrap(),
                            Expression::Literal(Literal::Integer(BigInt::from(-1))),
                        ));
                        lst.push((get_operator("*").unwrap(), expression));
                    }
                    SingleOperation::Not => {
                        lst.push((
                            get_operator("+").unwrap(),
                            Expression::SingleOperation(SingleOperation::Not, Box::new(expression)),
                        ));
                    }
                    SingleOperation::Spread => {
                        lst.push((
                            get_operator("+").unwrap(),
                            Expression::SingleOperation(
                                SingleOperation::Spread,
                                Box::new(expression),
                            ),
                        ));
                    }
                    _ => unreachable!("Other single operations are not allowed here"),
                }
            }
            let (input, lst) = many1(parse_next_with_operator)(input)?;
            for (operator, before_operator_opt, expression) in lst {
                if let Some(before_operator) = before_operator_opt {
                    match before_operator {
                        SingleOperation::Unary => {
                            lst.push((
                                get_operator("+").unwrap(),
                                Expression::Literal(Literal::Integer(BigInt::from(-1))),
                            ));
                            lst.push((get_operator("*").unwrap(), expression));
                        }
                        SingleOperation::Not => {
                            lst.push((
                                get_operator("+").unwrap(),
                                Expression::SingleOperation(SingleOperation::Not, Box::new(expression)),
                            ));
                        }
                        SingleOperation::Spread => {
                            lst.push((
                                get_operator("+").unwrap(),
                                Expression::SingleOperation(
                                    SingleOperation::Spread,
                                    Box::new(expression),
                                ),
                            ));
                        }
                        _ => unreachable!("Other single operations are not allowed here"),
                    }
                }
                lst.push((operator, expression));
            }
        };

        struct CurrentState<'a> {
            pub left_tree: Expression,
            pub operator: OrderedOperator<'a>,
            pub right_tree: Expression,
        }
        5
    }
}
