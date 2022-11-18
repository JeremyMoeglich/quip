use crate::fst::Statement;

use super::statement::format_statement;

pub fn format_statements(statements: &[Statement]) -> String {
    let mut result = String::new();
    for statement in statements {
        result.push_str(&format_statement(statement));
    }
    result
}