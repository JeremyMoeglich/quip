use crate::fst::{Space, SpacePart};

fn trim_space_to(input: &Space, to: &str) -> String {
    let mut result = String::new();
    for space in *input {
        result.push_str(match &space {
            SpacePart::Whitespace(s) => "",
            SpacePart::SingleLineComment(s) => &format_single_line_comment(s),
            SpacePart::MultiLineComment(s) => &format_multi_line_comment(s),
        });
    }
    result
}

pub fn trim_space0(input: &Space) -> String {
    trim_space_to(input, "")
}

pub fn trim_space1(input: &Space) -> String {
    trim_space_to(input, " ")
}

pub fn get_space(input: &Space) -> String {
    let mut result = String::new();
    for space in *input {
        result.push_str(match &space {
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

pub fn limit_whitespace(input: &Space, require_newline: bool) -> String {
    let base_string = match require_newline {
        true => "\n",
        false => "",
    }
    .to_string();
    input.space.iter().fold(String::new(), |mut acc, space_part| {
        let new_space_part = match space_part {
            SpacePart::Whitespace(s) => match s.len() {
                0 => base_string,
                _ => {
                    let line_count = s.lines().count();
                    match line_count {
                        0 => base_string,
                        _ => "\n".repeat(line_count.min(3)),
                    }
                }
            },
            SpacePart::SingleLineComment(s) => format_single_line_comment(s),
            SpacePart::MultiLineComment(s) => format_multi_line_comment(s),
        };
        acc.push_str(&new_space_part);
        acc
    })
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
