use nom::branch::alt;

use crate::fst::Statement;

use super::{
    arguments::parse_arguments,
    code_block::parse_code_block,
    expression::parse_expression,
    lexer::TokenKind,
    utils::{opt_token, parse_ident, token, ws0, ws1, ParseResult, TokenSlice},
};

pub fn parse_statement<'a>(input: TokenSlice<'a>) -> ParseResult<'a, Statement> {
    alt((parse_extern, parse_function, |input| {
        let (input, start_expr) = parse_expression(input)?;
        let (input, semi) = opt_token(TokenKind::Semi)(input)?;
        if let Some(_) = semi {
            todo!()
        } else {
            Ok((input, Statement::ImplicitReturn { value: start_expr }))
        }
    }))(input)
}

fn parse_extern(input: TokenSlice) -> ParseResult<Statement> {
    let (input, _) = token(TokenKind::Extern)(input)?;
    let (input, space_extern_ident) = ws1(input)?;
    let (input, name) = parse_ident(input)?;
    let (input, space_ident_lparen) = ws0(input)?;
    let (input, _) = token(TokenKind::LParen)(input)?;
    let (input, space_lparen_arg1) = ws0(input)?;
    let (input, args) = parse_arguments(input)?;
    let (input, _) = token(TokenKind::RParen)(input)?;
    let (input, right_space) = ws0(input)?;
    Ok((
        input,
        Statement::Extern {
            space_extern_ident,
            name: name,
            space_ident_lparen,
            space_lparen_arg1,
            args: args,
            right_space,
        },
    ))
}

fn parse_function(input: TokenSlice) -> ParseResult<Statement> {
    let (input, _) = token(TokenKind::Fn)(input)?;
    let (input, space_fn_ident) = ws1(input)?;
    let (input, name) = parse_ident(input)?;
    let (input, space_ident_lparen) = ws0(input)?;
    let (input, _) = token(TokenKind::LParen)(input)?;
    let (input, space_lparen_arg1) = ws0(input)?;
    let (input, args) = parse_arguments(input)?;
    let (input, _) = token(TokenKind::RParen)(input)?;
    let (input, space_rparen_lbrace) = ws0(input)?;
    let (input, _) = token(TokenKind::LBrace)(input)?;
    let (input, space_lbrace_expr) = ws0(input)?;
    let (input, body) = parse_code_block(input)?;
    let (input, _) = token(TokenKind::RBrace)(input)?;
    let (input, right_space) = ws0(input)?;
    Ok((
        input,
        Statement::Function {
            space_fn_ident,
            name: name,
            space_ident_lparen,
            space_lparen_arg1,
            args: args,
            space_rparen_lbrace,
            space_lbrace_expr,
            body: body,
            right_space,
        },
    ))
}
