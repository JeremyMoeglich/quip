use pad::{Alignment, PadStr};

use super::utils::{locate, Location};
use crate::core::Span;

pub fn create_fancy_error(original: &str, error: nom::Err<nom::error::Error<Span>>) -> String {
    match error {
        nom::Err::Incomplete(_) => panic!("Streaming not supported"),
        nom::Err::Error(err) => {
            let span = err.input;
            let mut fancy_error = create_fancy_error_span(original, span);
            fancy_error.push_str(&format!("\n{}", err));
            fancy_error
        }
        nom::Err::Failure(_) => panic!("Failure not supported"),
    }
}

fn add_line_numbers(text: &str, to_map_locations: Vec<Location>) -> (String, Vec<Location>) {
    let max_line_number_width = text.lines().count().to_string().len();

    let get_line_prefix = |line_number| {
        format!("{} | ", line_number)
            .pad_to_width_with_alignment(max_line_number_width, Alignment::Right)
    };

    let format_line = |line: &str, line_number: usize| {
        let prefix = get_line_prefix(line_number);
        format!("{}{}", prefix, line)
    };

    let new_text = text
        .lines()
        .enumerate()
        .map(|(i, line)| format_line(line, i + 1))
        .collect::<Vec<_>>()
        .join("\n");

    let new_locations = to_map_locations
        .iter()
        .map(|location| {
            let prefix_length = get_line_prefix(location.line + 1).len();
            Location {
                line: location.line,
                column: location.column + prefix_length,
            }
        })
        .collect::<Vec<_>>();

    (new_text, new_locations)
}

fn add_line_numbers_single(text: &str, to_map_location: Location) -> (String, Location) {
    let (new_text, new_locations) = add_line_numbers(text, vec![to_map_location]);
    (new_text, new_locations[0])
}

fn mark_code(text: &str, error_char_index: usize, padding: usize) -> String {
    // mark a line of code with a ^ at the start of the error and show the nearest {padding} lines
    let (text, location) = add_line_numbers_single(text, locate(text, error_char_index));

    let lines = text.lines().collect::<Vec<&str>>();

    let start_line = if location.line > padding {
        location.line - padding
    } else {
        0
    };
    let end_line = if location.line + padding < lines.len() {
        location.line + padding + 1
    } else {
        lines.len()
    };

    let before_lines = lines[start_line..location.line].join("\n");
    let after_lines = lines[location.line + 1..end_line].join("\n");

    let marked_line = lines[location.line].to_string();
    let marker_line = format!("{:width$}^", "", width = location.column);

    format!(
        "{}\n{}\n{}\n{}\n{}",
        before_lines, marked_line, marker_line, after_lines, ""
    )
}

pub fn create_fancy_error_span(original: &str, span: Span) -> String {
    let location = locate(original, span.location_offset());
    let line = location.line + 1;
    let column = location.column + 1;
    let mut fancy_error = format!("Syntax Error at line {}, column {}:\n", line, column);

    let padding = 2;

    let marked = mark_code(original, span.location_offset(), padding);

    fancy_error.push_str(&marked);

    fancy_error
}
