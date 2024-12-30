use crate::{
    block::parse_block,
    expression::parse_expression,
    utils::{opt, ws0, ws1},
};
use fst::{Expression, Statement};
use parser_core::*;

fn parse_if_block<'a>(input: Span<'a>) -> ParserResult<'a, (Expression, Vec<Statement>)> {
    let (input, _) = parse_if(input)?;
    let (input, _) = ws0(input);
    let (input, condition) = parse_expression(input)?;
    let (input, _) = ws0(input);
    let (input, code) = parse_block(input)?;
    Ok((input, (condition, code)))
}

pub fn parse_if_expr<'a>(input: Span<'a>) -> ParserResult<'a, Expression> {
    let (input, first_block) = parse_if_block(input)?;
    let (input, else_if_blocks) =
        many0((ws0, parse_else, ws1, parse_if_block).tuple().map(|e| e.3))(input);
    let (input, else_block) = opt((ws0, parse_else, ws1, parse_block).tuple().map(|v| v.3))(input);

    let mut blocks = vec![first_block];
    blocks.extend(else_if_blocks);

    Ok((input, Expression::If { blocks, else_block }))
}
