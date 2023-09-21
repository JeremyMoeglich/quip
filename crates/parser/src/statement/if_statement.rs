use ast::{CodeBlock, Expression, Statement};
use parser_core::*;
use crate::{
    block::parse_block,
    expression::parse_expression,
    utils::{ws0, ws1, opt},
};

fn parse_if_block<'a>(input: &Span<'a>) -> ParserResult<'a, (Expression, CodeBlock), TokenParserError> {
    let (input, _) = token_parser!(nodata If)(&input)?;
    let (input, _) = ws1(&input)?;
    let (input, condition) = parse_expression(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, code) = parse_block(&input)?;
    Ok((input, (condition, code)))
}

pub fn parse_if_statement<'a>(input: &Span<'a>) -> ParserResult<'a, Statement, TokenParserError> {
    let (input, first_block) = parse_if_block(&input)?;
    let (input, else_if_blocks) =
        many0(
            (ws0, token_parser!(nodata Else), ws1, parse_if_block).map(|e| e.3)
        )(&input)?;
    let (input, else_block) = opt(
        (ws0, token_parser!(nodata Else), ws1, parse_block).map(|v| {
            match v {
                Some((_, _, _, block)) => Some(block),
                None => None,
            }
        })
    )(&input)?;

    let mut else_tree = else_block.unwrap_or_else(CodeBlock::new);

    for (condition, code) in else_if_blocks.into_iter().rev() {
        else_tree = vec![Statement::If(condition, code, else_tree)];
    }

    Ok((input, Statement::If(first_block.0, first_block.1, else_tree)))
}
