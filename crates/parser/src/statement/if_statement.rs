use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::{
    ast::{CodeBlock, Expression, Statement},
    block::parse_block,
    expression::parse_expression,
    utils::{ws, ws1, Span},
};

fn parse_if_block(input: Span) -> IResult<Span, (Expression, CodeBlock)> {
    let (input, _) = tag("if")(input)?;
    let (input, _) = ws1(input)?;
    let (input, condition) = parse_expression(input)?;
    let (input, _) = ws(input)?;
    let (input, code) = parse_block(input)?;
    Ok((input, (condition, code)))
}

pub fn parse_if_statement(input: Span) -> IResult<Span, Statement> {
    let (input, first_block) = parse_if_block(input)?;
    let (input, else_if_blocks) =
        many0(map(tuple((ws, tag("else"), ws1, parse_if_block)), |e| e.3))(input)?;
    let (input, else_block) = map(
        opt(tuple((ws, tag("else"), ws1, parse_block))),
        |v| match v {
            Some((_, _, _, block)) => Some(block),
            None => None,
        },
    )(input)?;

    let mut else_tree = match else_block {
        Some(block) => block,
        None => CodeBlock::new(),
    };

    for (condition, code) in else_if_blocks.into_iter().rev() {
        else_tree = vec![Statement::If(condition, code, else_tree)];
    }

    Ok((
        input,
        Statement::If(first_block.0, first_block.1, else_tree),
    ))
}
