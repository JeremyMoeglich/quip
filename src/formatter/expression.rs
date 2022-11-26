use crate::fst::{
    Argument, Arguments, CallExpression, Expression, IdentSegment, NumberSegment, Segment, Space,
};

use super::utils::{
    format_separated, limit_whitespace, trim_space0, Delimiter, Separated, Separator,
};

pub fn format_expression(expr: &Expression) -> String {
    match expr {
        Expression::Call(call) => format_call(call),
        Expression::Segment(segment) => format_segment(segment),
    }
}

impl Separated<String> for Argument {
    fn text(&self) -> String {
        format_expression(&self.expr)
    }
    fn space(&self) -> Option<&Space> {
        Some(&self.space_ident_right)
    }
    fn after_comma(&self) -> &Option<Space> {
        &self.space_after_comma
    }
}

fn format_arguments(begin_space: &Space, args: &Arguments) -> String {
    format_separated(Delimiter::Parens, Separator::Comma, args, begin_space)
}

fn format_call(call: &CallExpression) -> String {
    format!(
        "{}{}{}{}",
        call.name,
        trim_space0(&call.space_ident_lparen),
        format_arguments(&call.space_lparen_arg1, &call.args),
        limit_whitespace(&call.right_space, false)
    )
}

fn format_spaced<T>(spaced: T) -> String
where
    T: Spaced,
{
    format!("{}{}", spaced.value(), trim_space0(&spaced.space()))
}

trait Spaced {
    fn value(&self) -> String;
    fn space(&self) -> Space;
}

impl Spaced for &NumberSegment {
    fn value(&self) -> String {
        self.number.to_string()
    }
    fn space(&self) -> Space {
        self.right_space.clone()
    }
}

impl Spaced for &IdentSegment {
    fn value(&self) -> String {
        self.ident.to_string()
    }
    fn space(&self) -> Space {
        self.right_space.clone()
    }
}

fn format_segment(segment: &Segment) -> String {
    match segment {
        Segment::Ident(ident_segment) => format_spaced(ident_segment),
        Segment::Number(number_segment) => format_spaced(number_segment),
    }
}
