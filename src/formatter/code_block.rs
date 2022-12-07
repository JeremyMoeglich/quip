use crate::fst::CodeBlock;

use super::utils::{format_separated, trim_space0, Delimiter, Formatable, Separator};

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
