use crate::fst::{Space, SpacePart};

pub fn trim_space0(input: &Space) -> String {
    let mut result = String::new();
    for space in input {
        result.push_str(match space {
            SpacePart::Whitespace(s) => "",
            SpacePart::SingleLineComment(s) => &format_single_line_comment(s),
            SpacePart::MultiLineComment(s) => &format_multi_line_comment(s),
        });
    }
    result
}

pub fn get_space(input: &Space) -> String {
    let mut result = String::new();
    for space in input {
        result.push_str(match space {
            SpacePart::Whitespace(s) => s,
            SpacePart::SingleLineComment(s) => &format_single_line_comment(s),
            SpacePart::MultiLineComment(s) => &format_multi_line_comment(s),
        });
    }
    result
}

pub fn format_single_line_comment(input: &str) -> String {
    let mut text = "// ".to_string();
    text.push_str(input.trim());
    text.push_str("\n");
    text
}

pub fn format_multi_line_comment(input: &str) -> String {
    let mut text = "/* ".to_string();
    text.push_str(input.trim());
    text.push_str(" */\n");
    text
}

pub fn limit_whitespace(input: &str) -> String {
    match input.len() {
        0 => "".to_string(),
        _ => {
            let line_count = input.lines().count();
            match line_count {
                0 => " ".to_string(),
                _ => "\n".repeat(line_count.min(2)),
            }
        }
    }
}

pub fn indent(input: &str, indent: usize) -> String {
    let single_indent = " ".repeat(4);
    let indent = single_indent.repeat(indent);
    input
        .lines()
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<String>>()
        .join("\n")
}
