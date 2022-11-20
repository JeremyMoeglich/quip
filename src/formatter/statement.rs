use crate::fst::{
    Expression, ExternStatement, FunctionStatement, Parameter, Parameters, Space, Statement,
    EMPTY_SPACE,
};

use super::{
    expression::format_expression,
    utils::{
        format_separated, limit_whitespace, trim_space0, trim_space1, Delimiter, Formatable,
        Separated, Separator,
    },
};

impl Formatable for &Statement {
    fn format(&self) -> String {
        match self {
            Statement::Extern(extern_statement) => format_extern(extern_statement),
            Statement::ImplicitReturn(expr) => format_implicit_return(expr),
            Statement::Function(function) => format_function(function),
        }
    }
}

impl Formatable for Statement {
    fn format(&self) -> String {
        (&self).format()
    }
}

fn format_extern(extern_: &ExternStatement) -> String {
    format!(
        "extern{}{}{}{}{}",
        trim_space1(&extern_.space_extern_ident),
        extern_.name,
        trim_space0(&extern_.space_ident_lparen),
        format_parameters(&extern_.space_lparen_arg1, &extern_.params),
        limit_whitespace(&extern_.right_space, true),
    )
}

fn format_implicit_return(expr: &Expression) -> String {
    format_expression(expr)
}

fn format_parameters(begin_space: &Space, params: &Parameters) -> String {
    format_separated(Delimiter::Parens, Separator::Comma, params, begin_space)
}

impl Separated<String> for Parameter {
    fn text(&self) -> String {
        self.name.clone()
    }
    fn space(&self) -> &Space {
        &self.space_ident_right
    }
    fn after_comma(&self) -> &Option<Space> {
        &self.space_after_comma
    }
}

fn format_function(func: &FunctionStatement) -> String {
    format!(
        "fn{}{}{}{}{}{}{}{}",
        trim_space1(&func.space_fn_ident),
        func.name,
        trim_space0(&func.space_ident_lparen),
        format_parameters(&func.space_lparen_arg1, &func.params),
        trim_space0(&func.space_rparen_lbrace),
        trim_space1(&func.space_lbrace_expr),
        (&func.body).format(),
        limit_whitespace(&func.right_space, true),
    )
}

impl Separated<Statement> for Statement {
    fn text(&self) -> Self {
        self.clone()
    }
    fn space(&self) -> &Space {
        &EMPTY_SPACE
    }
    fn after_comma(&self) -> &Option<Space> {
        &None
    }
}
