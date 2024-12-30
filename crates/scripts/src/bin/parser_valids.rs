use parser::{core::create_span, parse_file};
use scripts::fuzz_possibilities;
use std::{fs::OpenOptions, io::Write, hash::Hasher};

fn sequence_id<'a>(tokens: impl Iterator<Item = &'a str>) -> u64 {
    use twox_hash::XxHash64;
    let mut hasher = XxHash64::default();
    hasher.write(b"sequence");
    for token in tokens {
        hasher.write(token.as_bytes());
    }
    hasher.finish()
}

fn main() {
    let file_path = "parser_valids.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    fuzz_possibilities(|new_tokens| {
        let id = sequence_id(new_tokens.iter().map(|token| token.kind().kind_name()));
        let mut contents = String::new();
        contents.push_str(&format!("Sequence ID: {}\n", id));
        contents.push_str(
            &new_tokens
                .iter()
                .map(|token| token.kind().kind_name())
                .collect::<Vec<_>>()
                .join(", "),
        );
        contents.push_str("\n\n");

        let span = create_span(new_tokens);
        if let Ok((_, parsed)) = parse_file(span) {
            contents.push_str(&format!("{:#?}", parsed));
            contents.push_str("\n\n---\n\n");
            file.write_all(contents.as_bytes()).unwrap();
        }
    });
}
