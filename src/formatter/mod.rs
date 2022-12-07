use crate::fst::Fst;

use self::{lst_statement::format_statements, utils::trim_space0};

mod code_block;
mod expression;
mod lst_statement;
mod statement;
mod utils;

pub fn format_fst(fst: &Fst) -> String {
    let mut result = String::new();
    result.push_str(&trim_space0(&fst.beginning_space));
    result.push_str(&format_statements(&fst.index_block));
    result
}
