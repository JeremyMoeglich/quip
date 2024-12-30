use fst::Expression;
use parser_core::*;

use crate::{
    block::parse_righthand_block,
    destructure::parse_mutable_destructure,
    utils::{opt, ws0},
};

use super::parse_expression;

fn parse_optional_label<'a>(input: Span<'a>) -> SafeParserResult<'a, Option<&'a str>> {
    opt((parse_label, ws0).tuple().map(|(label, _)| label))(input)
}

pub fn parse_while_expr<'a>(input: Span<'a>) -> ParserResult<'a, Expression> {
    let (input, label) = parse_optional_label(input);
    let (input, _) = parse_while(input)?;
    let (input, _) = ws0(input);
    let (input, condition) = parse_expression(input)?;
    let (input, _) = ws0(input);
    let (input, (token, source_span)) = input.take_token();
    let (input, body) = match token.delocate() {
        Some(Token::Do) => {
            let (input, _) = ws0(input);
            let (input, body) = parse_expression(input)?;
            (input, body)
        }
        Some(Token::LeftBrace) => {
            let (input, _) = ws0(input);
            let (input, statements) = parse_righthand_block(input)?;
            (
                input,
                Expression::Block {
                    environment: None,
                    block: statements,
                },
            )
        }
        _ => {
            return Err(token.as_parser_error(TokenKind::Do | TokenKind::LeftBrace, source_span));
        }
    };
    let (input, _) = ws0(input);

    if let Ok((input, _)) = parse_else(input) {
        let (input, _) = ws0(input);
        let (input, else_block) = parse_expression(input)?;
        return Ok((
            input,
            Expression::While {
                label: label.map(|s| s.to_string()),
                condition: Box::new(condition),
                body: Box::new(body),
                else_block: Some(Box::new(else_block)),
            },
        ));
    }

    Ok((
        input,
        Expression::While {
            label: label.map(|s| s.to_string()),
            condition: Box::new(condition),
            body: Box::new(body),
            else_block: None,
        },
    ))
}

pub fn parse_loop_expr<'a>(input: Span<'a>) -> ParserResult<'a, Expression> {
    let (input, label) = parse_optional_label(input);
    let (input, _) = parse_loop(input)?;
    let (input, _) = ws0(input);
    let (input, expression) = parse_expression(input)?;

    Ok((
        input,
        Expression::Loop {
            label: label.map(|s| s.to_string()),
            body: Box::new(expression),
        },
    ))
}

pub fn parse_for_expr<'a>(input: Span<'a>) -> ParserResult<'a, Expression> {
    let (input, label) = parse_optional_label(input);
    let (input, _) = parse_for(input)?;
    let (input, _) = ws0(input);
    let (input, destructure) = parse_mutable_destructure(input)?;
    let (input, _) = ws0(input);
    let (input, _) = parse_in(input)?;
    let (input, _) = ws0(input);
    let (input, iterator) = parse_expression(input)?;
    let (input, _) = ws0(input);
    let (input, (token, source_span)) = input.take_token();
    let (input, body) = match token.delocate() {
        Some(Token::Do) => {
            let (input, _) = ws0(input);
            let (input, body) = parse_expression(input)?;
            (input, body)
        }
        Some(Token::LeftBrace) => {
            let (input, _) = ws0(input);
            let (input, statements) = parse_righthand_block(input)?;
            (
                input,
                Expression::Block {
                    environment: None,
                    block: statements,
                },
            )
        }
        _ => {
            return Err(token.as_parser_error(TokenKind::Do | TokenKind::LeftBrace, source_span));
        }
    };
    let (input, _) = ws0(input);

    if let Ok((input, _)) = parse_else(input) {
        let (input, _) = ws0(input);
        let (input, else_block) = parse_expression(input)?;
        return Ok((
            input,
            Expression::For {
                label: label.map(|s| s.to_string()),
                destructure,
                iterator: Box::new(iterator),
                body: Box::new(body),
                else_block: Some(Box::new(else_block)),
            },
        ));
    }

    Ok((
        input,
        Expression::For {
            label: label.map(|s| s.to_string()),
            destructure,
            iterator: Box::new(iterator),
            body: Box::new(body),
            else_block: None,
        },
    ))
}
