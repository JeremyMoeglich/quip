use std::fmt::Display;

use crate::fst::{Space, SpacePart};

fn trim_space_to(input: &Space, to: &str) -> String {
    let mut result = String::new();
    for space in input {
        let string = match &space {
            SpacePart::Whitespace(_) => to.to_string(),
            SpacePart::SingleLineComment(s) => format_single_line_comment(s),
            SpacePart::MultiLineComment(s) => format_multi_line_comment(s),
        };
        result.push_str(&string);
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
    for space in input {
        let string = match &space {
            SpacePart::Whitespace(s) => s.to_string(),
            SpacePart::SingleLineComment(s) => format_single_line_comment(s),
            SpacePart::MultiLineComment(s) => format_multi_line_comment(s),
        };
        result.push_str(&string);
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

fn escape_all(input: &str) -> String {
    // escape characters such as newlines as \n
    let mut result = String::new();
    for c in input.chars() {
        match c {
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result
}

fn has_newline(input: &str) -> bool {
    input.contains('\r') || input.contains('\n')
}

pub fn limit_whitespace(input: &Space, require_newline: bool) -> String {
    let text = input
        .space
        .iter()
        .fold(String::new(), |mut acc, space_part| {
            let new_space_part = match space_part {
                SpacePart::Whitespace(s) => match s.len() {
                    0 => "".to_string(),
                    _ => {
                        let line_count = s.lines().count();
                        match line_count {
                            0 => unreachable!("Whitespace should have at least one line"),
                            _ => "\n".repeat(line_count.min(3)),
                        }
                    }
                },
                SpacePart::SingleLineComment(s) => format_single_line_comment(s),
                SpacePart::MultiLineComment(s) => format_multi_line_comment(s),
            };
            acc.push_str(&new_space_part);
            acc
        });
    if require_newline && !has_newline(&text) {
        format!("\n{}", text)
    } else {
        text
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

#[derive(Debug, Clone, PartialEq)]
pub enum Delimiter {
    Brackets,
    Braces,
    Parens,
}

impl Delimiter {
    pub fn start(&self) -> &'static str {
        match self {
            Delimiter::Brackets => "[",
            Delimiter::Braces => "{",
            Delimiter::Parens => "(",
        }
    }
    pub fn end(&self) -> &'static str {
        match self {
            Delimiter::Brackets => "]",
            Delimiter::Braces => "}",
            Delimiter::Parens => ")",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Separator {
    Comma,
    Newline,
}

impl Display for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Separator::Comma => write!(f, ","),
            Separator::Newline => write!(f, ""),
        }
    }
}

pub fn format_separated<S: Separated<T>, T: Formatable>(
    delimiter: Delimiter,
    separator: Separator,
    separated: &[S],
    begin_space: &Space,
) -> String {
    let formated = separated
        .iter()
        .map(|s| (s.text().format(), s.space(), s.after_comma()))
        .collect::<Vec<_>>();
    let seperator_ref = &separator;
    let render_texts = move |trailing: bool| {
        formated
            .iter()
            .enumerate()
            .map(|(i, s)| {
                format!(
                    "{}{}{}{}",
                    s.0,
                    match i == separated.len() - 1 {
                        false => seperator_ref.to_string(),
                        true => match trailing {
                            true => seperator_ref.to_string(),
                            false => "".to_string(),
                        },
                    },
                    match s.1 {
                        Some(s) => trim_space0(s),
                        None => "".to_string(),
                    },
                    match s.2 {
                        None => "".to_string(),
                        Some(space) => trim_space0(space),
                    }
                )
            })
            .collect::<Vec<String>>()
    };
    let render_ref = &render_texts;
    let delimiter_ref = &delimiter;
    let render_single_line = move || {
        format!(
            "{}{}{}",
            delimiter_ref.start(),
            render_ref(false).join(" "),
            delimiter_ref.end()
        )
    };
    let render_multi_line = move || {
        format!(
            "{}\n{}\n{}",
            delimiter_ref.start(),
            indent(&render_ref(true).join("\n"), 1),
            delimiter_ref.end()
        )
    };
    if begin_space.has_comments()
        || separated.iter().any(|s| match s.space() {
            Some(space) => {
                space.has_comments()
                    || match s.after_comma() {
                        None => false,
                        Some(space) => space.has_comments(),
                    }
            }
            None => separator == Separator::Newline,
        })
    {
        return render_multi_line();
    };
    let text = render_single_line();
    if text.len() > 15 {
        render_multi_line()
    } else {
        text
    }
}

pub trait Separated<T>
where
    T: Formatable,
{
    fn text(&self) -> T;
    fn space(&self) -> Option<&Space>;
    fn after_comma(&self) -> &Option<Space>;
}

pub trait Formatable {
    fn format(&self) -> String;
}

impl Formatable for String {
    fn format(&self) -> String {
        self.clone()
    }
}
