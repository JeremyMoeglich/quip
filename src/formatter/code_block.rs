use crate::fst::CodeBlock;

use super::{lst_statement::format_statements, utils::indent};

pub fn format_code_block(code_block: &CodeBlock) -> String {
    let mut result = "{\n".to_string();
    let body = indent(&format_statements(code_block).as_str(), 1);
    result.push_str(&body);
    result.push_str("}");
    result
}
