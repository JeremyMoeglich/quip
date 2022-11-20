use crate::fst::Statement;

use super::utils::Formatable;

pub fn format_statements(statements: &[Statement]) -> String {
    let mut result = String::new();
    for statement in statements {
        result.push_str(&statement.format());
    }
    result
}