use fst::{Expression, LabelExpression, Statement, Whitespace0, Whitespace1};
use parser_core::*;

use crate::{
    expression::parse_expression,
    utils::{opt, ws0, ws1},
};

use super::semicolon::opt_semicolon;

fn parse_label_expression<'a>(input: Span<'a>) -> ParserResult<'a, LabelExpression> {
    if let Ok((input, pre_space)) = ws1(input) {
        if let Ok((input, label)) = parse_label(input) {
            if let Ok((input, post_label_space)) = ws1(input) {
                if let Ok((input, value)) = parse_expression(input) {
                    let (input, semi_space) = ws0(input);
                    let (input, _) = parse_semicolon(input)?;
                    return Ok((
                        input,
                        LabelExpression::WithExpression {
                            pre_space,
                            label: Some((label.to_string(), post_label_space)),
                            expr: value,
                            semi_space,
                        },
                    ));
                }
            }
            let (input, semi_space) =
                opt((ws0, parse_semicolon).tuple().map(|(space, _)| space))(input);
            return Ok((
                input,
                LabelExpression::JustLabel {
                    pre_space,
                    label: label.to_string(),
                    semi_space,
                },
            ));
        } else {
            if let Ok((input, value)) = parse_expression(input) {
                let (input, semi_space) = ws0(input);
                let (input, _) = parse_semicolon(input)?;
                return Ok((
                    input,
                    LabelExpression::WithExpression {
                        pre_space,
                        expr: value,
                        semi_space,
                        label: None,
                    },
                ));
            }
        }
    }
    let (input, semi_space) = opt((ws0, parse_semicolon).tuple().map(|(space, _)| space))(input);
    Ok((input, LabelExpression::Nothing { semi_space }))
}

pub fn parse_break_statement<'a>(input: Span<'a>) -> ParserResult<'a, Statement> {
    let (input, _) = parse_break(input)?;
    let (input, (label, value)) = parse_label_expression(input)?;
    Ok((input, Statement::Break { label, value }))
}

pub fn parse_return_statement<'a>(input: Span<'a>) -> ParserResult<'a, Statement> {
    let (input, _) = parse_return(input)?;
    let (input, (label, value)) = parse_label_expression(input)?;
    Ok((input, Statement::Return { label, value }))
}

pub fn parse_continue_statement<'a>(input: Span<'a>) -> ParserResult<'a, Statement> {
    // continue aren't allowed to have a value, but may have a label
    let (input, _) = parse_continue(input)?;
    if let Ok((input, _)) = ws1(input) {
        if let Ok((input, label)) = parse_label(input) {
            let (input, _) = opt_semicolon(input);
            return Ok((
                input,
                Statement::Continue {
                    label: Some(label.to_string()),
                },
            ));
        }
    }
    let (input, _) = opt_semicolon(input);
    Ok((input, Statement::Continue { label: None }))
}
