mod control_stmt;
mod enum_stmt;
mod env_stmt;
mod function_stmt;
mod impl_stmt;
mod import_stmt;
mod module_stmt;
mod semicolon;
mod struct_stmt;
mod trait_stmt;

use control_stmt::{parse_break_statement, parse_continue_statement, parse_return_statement};
use env_stmt::parse_env_statement;
use fst::Statement;
use function_stmt::parse_function_statement;
use impl_stmt::parse_impl_statement;
use module_stmt::parse_module_statement;
use parser_core::*;
use trait_stmt::parse_trait_statement;

use crate::utils::{opt, opt_bool};

use self::{
    enum_stmt::parse_enum_statement, import_stmt::parse_import_statement,
    struct_stmt::parse_struct_statement,
};

use super::{expression::parse_expression, utils::ws0};

pub fn parse_statement<'a>(input: Span<'a>) -> ParserResult<'a, Statement> {
    match (
        parse_return_statement,
        parse_break_statement,
        parse_continue_statement,
        parse_function_statement,
        parse_struct_statement,
        parse_enum_statement,
        parse_trait_statement,
        parse_impl_statement,
        parse_import_statement,
        parse_module_statement,
        parse_env_statement,
    )
        .alt()(input)
    {
        Ok((input, statement)) => Ok((input, statement)),
        Err(statement_parse_error) => match parse_expression(input) {
            Ok((input, expression)) => {
                let (input, has_semi) = opt_bool((ws0, parse_semicolon).tuple())(input);
                Ok((
                    input,
                    Statement::Expression {
                        expr: expression,
                        semi: !has_semi,
                    },
                ))
            }
            Err(expression_parse_error) => {
                Err(statement_parse_error.accumulate(expression_parse_error))
            }
        },
    }
}
