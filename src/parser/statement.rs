use crate::fst::{ExternStatement, FunctionStatement, Statement};

use super::{
    code_block::parse_code_block,
    common::{opt_token, parse_ident, token, ws0, ws1},
    core::{ParseResult, Parser, TokenSlice},
    expression::parse_expression,
    lexer::TokenKind,
    parameters::parse_parameters,
};

pub fn parse_statement<'a>(input: TokenSlice<'a>) -> ParseResult<'a, Statement> {
    parse_extern.alt(parse_function).alt(
        parse_expression
            .chain(opt_token(TokenKind::Semi))
            .map_result(|(expr, semi)| match semi {
                Some(_semi) => todo!(),
                None => Statement::ImplicitReturn(expr),
            }),
    )(input)
}

fn parse_extern(input: TokenSlice) -> ParseResult<Statement> {
    token(TokenKind::Extern)
        .chain(&ws1)
        .chain(parse_ident)
        .chain(&ws0)
        .chain(token(TokenKind::LParen))
        .chain(&ws0)
        .chain(parse_parameters)
        .chain(token(TokenKind::RParen))
        .chain(&ws0)
        .flattened()
        .map_result(
            |(
                _,
                space_extern_ident,
                name,
                space_ident_lparen,
                _,
                space_lparen_arg1,
                params,
                _,
                right_space,
            )| {
                Statement::Extern(ExternStatement {
                    space_extern_ident,
                    name,
                    space_ident_lparen,
                    space_lparen_arg1,
                    params,
                    right_space,
                })
            },
        )(input)
}

fn parse_function(input: TokenSlice) -> ParseResult<Statement> {
    token(TokenKind::Fn)
        .chain(&ws1)
        .chain(&parse_ident)
        .chain(&ws0)
        .chain(token(TokenKind::LParen))
        .chain(&ws0)
        .chain(&parse_parameters)
        .chain(token(TokenKind::RParen))
        .chain(&ws0)
        .chain(&parse_code_block)
        .chain(&ws0)
        .flattened()
        .map_result(
            |(
                _,
                space_fn_ident,
                name,
                space_ident_lparen,
                _,
                space_lparen_arg1,
                params,
                _,
                space_rparen_lbrace,
                body,
                right_space,
            )| {
                Statement::Function(FunctionStatement {
                    space_fn_ident,
                    name: name,
                    space_ident_lparen,
                    space_lparen_arg1,
                    params,
                    space_rparen_lbrace,
                    body,
                    right_space,
                })
            },
        )(input)
}
