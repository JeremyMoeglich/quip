mod assignment;
mod declaration;
mod enum_statement;
mod function;
mod generic;
mod if_statement;
mod impl_statement;
mod struct_statement;

use ast::{Statement, StatementInner};
use parser_core::*;

use crate::utils::opt;

use self::{
    assignment::parse_assignment, declaration::parse_declaration, enum_statement::parse_enum,
    function::parse_function, if_statement::parse_if_statement, impl_statement::parse_impl,
    struct_statement::parse_struct,
};

use super::{block::parse_block, expression::parse_expression, utils::ws0};

pub fn parse_statement<'a>(input: &Span<'a>) -> ParserResult<'a, Statement> {
    (
        ws0,
        (
            parse_function,
            parse_if_statement,
            parse_declaration,
            parse_block.map(|block| StatementInner::Scope { body: block }),
            parse_assignment,
            parse_struct,
            parse_impl,
            parse_enum,
            parse_expression.map(|expression| StatementInner::Expression { expr: expression }),
        )
            .alt(),
        ws0,
        opt(token_parser!(nodata Semicolon)),
    )
        .tuple()
        .map(|(_, statement, _, semicolon)| Statement {
            inner: statement,
            returned: match semicolon {
                Some(_) => false,
                None => true,
            },
        })(input)
}
