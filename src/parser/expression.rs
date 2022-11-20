use crate::fst::{CallExpression, Expression, IdentSegment, NumberSegment, Segment};

use super::{
    arguments::parse_arguments,
    lexer::TokenKind,
    utils::{parse_ident, token, ws0, ParseResult, TokenSlice},
};
use nom::{branch::alt, combinator::map, sequence::tuple};

pub fn parse_expression(input: TokenSlice) -> ParseResult<Expression> {
    alt((
        parse_call,
        map(parse_segment, |segment| Expression::Segment(segment)),
    ))(input)
}

pub fn parse_call(input: TokenSlice) -> ParseResult<Expression> {
    let (input, name) = parse_ident(input)?;
    let (input, space_ident_lparen) = ws0(input)?;
    let (input, _) = token(TokenKind::LParen)(input)?;
    let (input, space_lparen_arg1) = ws0(input)?;
    let (input, args) = parse_arguments(input)?;
    let (input, _) = token(TokenKind::RParen)(input)?;
    let (input, right_space) = ws0(input)?;
    Ok((
        input,
        Expression::Call(CallExpression {
            name,
            space_ident_lparen,
            space_lparen_arg1,
            args,
            right_space,
        }),
    ))
}

fn parse_number(input: TokenSlice) -> ParseResult<Segment> {
    let (input, number) = token(TokenKind::Number)(input)?;
    let (input, right_space) = ws0(input)?;
    Ok((
        input,
        Segment::Number(NumberSegment {
            number: number.string(),
            right_space,
        }),
    ))
}

fn parse_segment(input: TokenSlice) -> ParseResult<Segment> {
    alt((
        parse_number,
        map(tuple((parse_ident, ws0)), |(ident, right_space)| {
            Segment::Ident(IdentSegment { ident, right_space })
        }),
    ))(input)
}
