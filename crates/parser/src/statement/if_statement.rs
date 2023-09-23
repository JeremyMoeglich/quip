use crate::{
    block::parse_block,
    expression::parse_expression,
    utils::{opt, ws0, ws1},
};
use ast::{CodeBlock, Expression, Statement, StatementInner};
use parser_core::*;

fn parse_if_block<'a>(input: &Span<'a>) -> ParserResult<'a, (Expression, CodeBlock)> {
    let (input, _) = token_parser!(nodata If)(&input)?;
    let (input, _) = ws1(&input)?;
    let (input, condition) = parse_expression(&input)?;
    let (input, _) = ws0(&input)?;
    let (input, code) = parse_block(&input)?;
    Ok((input, (condition, code)))
}

pub fn parse_if_statement<'a>(input: &Span<'a>) -> ParserResult<'a, StatementInner> {
    let (input, first_block) = parse_if_block(&input)?;
    let (input, else_if_blocks) = many0(
        (ws0, token_parser!(nodata Else), ws1, parse_if_block)
            .tuple()
            .map(|e| e.3),
    )(&input)?;
    let (input, else_block) = opt((ws0, token_parser!(nodata Else), ws1, parse_block)
        .tuple()
        .map(|v| v.3))(&input)?;

    let mut else_tree = else_block.unwrap_or(CodeBlock { statements: vec![] });

    for (condition, code) in else_if_blocks.into_iter().rev() {
        else_tree = CodeBlock {
            statements: vec![Statement {
                inner: StatementInner::If {
                    condition,
                    then_block: code,
                    else_block: else_tree,
                },
                returned: false,
            }],
        };
    }

    Ok((
        input,
        StatementInner::If {
            condition: first_block.0,
            then_block: first_block.1,
            else_block: else_tree,
        },
    ))
}
