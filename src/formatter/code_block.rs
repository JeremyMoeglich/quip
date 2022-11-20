use crate::fst::CodeBlock;

use super::utils::{format_separated, Delimiter, Formatable, Separator, trim_space0};

impl Formatable for &CodeBlock {
    fn format(&self) -> String {
        format!(
            "{}{}",
            format_separated(
                Delimiter::Braces,
                Separator::Newline,
                &self.statements,
                &self.space_brace_stat1,
            ),
            trim_space0(&self.right_space),
        )
    }
}
