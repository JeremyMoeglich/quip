use crate::fst::{Expression, Parameter, Segment};

use super::{
    lexer::TokenKind,
    utils::{comma_separated, parse_ident, token, ws0, ParseResult, TokenSlice},
};
use nom::{branch::alt, combinator::map, sequence::tuple};

pub fn parse_expression(input: TokenSlice) -> ParseResult<Expression> {
    alt((
        parse_call,
        map(parse_segment, |segment| Expression::Segment {
            segment: segment,
        }),
    ))(input)
}

pub fn parse_call(input: TokenSlice) -> ParseResult<Expression> {
    let (input, name) = parse_ident(input)?;
    let (input, space_ident_lparen) = ws0(input)?;
    let (input, _) = token(TokenKind::LParen)(input)?;
    let (input, space_lparen_arg1) = ws0(input)?;
    let (input, params) = comma_separated(parse_expression)(input)?;
    let (input, _) = token(TokenKind::RParen)(input)?;
    let (input, right_space) = ws0(input)?;
    Ok((
        input,
        Expression::Call {
            name,
            space_ident_lparen,
            space_lparen_arg1,
            params: params
                .into_iter()
                .map(|(param, space, second_space)| Parameter::new(param, space, second_space))
                .collect(),
            right_space,
        },
    ))
}

fn parse_number(input: TokenSlice) -> ParseResult<Segment> {
    let (input, number) = token(TokenKind::Number)(input)?;
    let (input, right_space) = ws0(input)?;
    Ok((
        input,
        Segment::Number {
            number: number.number(),
            right_space,
        },
    ))
}

fn parse_segment(input: TokenSlice) -> ParseResult<Segment> {
    alt((
        parse_number,
        map(tuple((parse_ident, ws0)), |(ident, right_space)| {
            Segment::Ident { ident, right_space }
        }),
    ))(input)
}
