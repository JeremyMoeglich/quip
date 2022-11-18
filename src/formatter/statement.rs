use crate::fst::{Argument, CodeBlock, Expression, Space, Statement};

use super::utils::{format_newline_whitespace, trim_space0, trim_space1};

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
            space_fn_identlet,
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

fn format_extern(
    space_extern_ident: &Space,
    name: &str,
    space_ident_lparen: &Space,
    space_lparen_arg1: &Space,
    args: &[Argument],
    right_space: &Space,
) -> String {
    format!(
        "extern{}{}{}{}{}",
        &trim_space1(space_extern_ident),
        name,
        &trim_space0(space_ident_lparen),
        format_arguments(space_lparen_arg1, args),
        &format_newline_whitespace(right_space),
    )
}

fn format_implicit_return(expr: &Expression) -> String {
    format_expression(expr)
}

fn format_function(
    space_fn_ident: &Space,
    name: &str,
    space_ident_lparen: &Space,
    space_lparen_arg1: &Space,
    args: &[Argument],
    space_rparen_lbrace: &Space,
    space_lbrace_expr: &Space,
    body: CodeBlock,
    right_space: &Space,
) {
    
}

fn format_arguments(begin_space: &Space, args: &[Argument]) -> String {
    let render_single_line = move || {
        let mut result = String::new();
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&format_argument(arg));
        }
        result
    };
    let render_multi_line = move || {
        let mut result = String::new();
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                result.push_str(",\n");
            }
            result.push_str(&format_argument(arg));
        }
        result
    };
    if begin_space.has_comments() {
        return render_multi_line();
    };
    let text = render_single_line();
    if text.len() > 20 {
        render_multi_line()
    } else {
        text
    }
}
