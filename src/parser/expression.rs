use crate::fst::{CallExpression, Expression, IdentSegment, NumberSegment, Segment};

use super::{
    arguments::parse_arguments,
    common::{parse_ident, token, ws0},
    core::{ParseResult, Parser, TokenSlice},
    lexer::TokenKind,
};

pub fn parse_expression(input: TokenSlice) -> ParseResult<Expression> {
    parse_call
        .alt(parse_segment.map_result(&Expression::Segment))
        .parse(input)
}

pub fn parse_call(input: TokenSlice) -> ParseResult<Expression> {
    parse_ident
        .chain(&ws0)
        .chain(&token(TokenKind::LParen))
        .chain(&ws0)
        .chain(&parse_arguments)
        .chain(&token(TokenKind::RParen))
        .chain(&ws0)
        .flattened()
        .map_result(
            &|(name, space_ident_lparen, _, space_lparen_arg1, args, _, right_space)| {
                Expression::Call(CallExpression {
                    name,
                    space_ident_lparen,
                    space_lparen_arg1,
                    args,
                    right_space,
                })
            },
        )(input)
}

fn parse_number(input: TokenSlice) -> ParseResult<Segment> {
    token(TokenKind::Number)
        .chain(&ws0)
        .map_result(&|(number, right_space)| {
            Segment::Number(NumberSegment {
                number: number.string(),
                right_space,
            })
        })(input)
}

fn parse_segment(input: TokenSlice) -> ParseResult<Segment> {
    parse_ident
        .chain(&ws0)
        .flattened()
        .map_result(&|(ident, right_space)| Segment::Ident(IdentSegment { ident, right_space }))
        .alt(parse_number)
        .parse(input)
}
