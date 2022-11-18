use crate::fst::Statement;

pub fn format_statement(statement: &Statement) -> String {
    let mut result = String::new();
    let text = match statement {
        Statement::Extern {
            space_extern_ident,
            name,
            space_ident_lparen,
            space_lparen_arg1,
            args,
            right_space,
        } => format_extern(
            space_extern_ident,
            name,
            space_ident_lparen,
            space_lparen_arg1,
            args,
            right_space,
        ),
        Statement::ImplicitReturn { value } => format_implicit_return(value),
        Statement::Function {
            space_fn_ident,
            name,
            space_ident_lparen,
            space_lparen_arg1,
            args,
            space_rparen_lbrace,
            space_lbrace_expr,
            body,
            right_space,
        } => format_function(
            space_fn_ident,
            name,
            space_ident_lparen,
            space_lparen_arg1,
            args,
            space_rparen_lbrace,
            space_lbrace_expr,
            body,
            right_space,
        ),
    };
}
