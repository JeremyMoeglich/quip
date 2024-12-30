use fst::Location;
use pad::{Alignment, PadStr};
use parser_core::LocatedParserError;

fn lines(text: &str) -> impl Iterator<Item = &str> {
    text.split('\n')
}

fn add_line_numbers(text: &str, to_map_locations: Vec<Location>) -> (String, Vec<Location>) {
    let max_line_number_width = lines(text).count().to_string().len();

    let get_line_prefix = |line_number| {
        format!("{} | ", line_number)
            .pad_to_width_with_alignment(max_line_number_width, Alignment::Right)
    };

    let format_line = |line: &str, line_number: usize| {
        let prefix = get_line_prefix(line_number);
        format!("{}{}", prefix, line)
    };

    let new_text = lines(text)
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
                index: location.index + prefix_length,
            }
        })
        .collect::<Vec<_>>();

    (new_text, new_locations)
}

fn add_line_numbers_single(text: &str, to_map_location: Location) -> (String, Location) {
    let (new_text, new_locations) = add_line_numbers(text, vec![to_map_location]);
    (new_text, new_locations[0])
}

fn mark_code(text: &str, location: Location, padding: usize) -> String {
    // mark a line of code with a ^ at the start of the error and show the nearest {padding} lines
    let (text, location) = add_line_numbers_single(text, location);

    let lines = lines(&text).collect::<Vec<&str>>();

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

pub fn create_fancy_error(original: &str, err: LocatedParserError) -> String {
    let padding = 2;

    let marked = mark_code(original, err.source_span.start, padding);

    format!("{}\n{}", err, marked)
}
