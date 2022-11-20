use quip::{formatter::format_fst, parser::parse};

fn main() {
    let code = include_str!("./example_code/1.kld");
    let fst = parse(code).unwrap();
    println!("{}", format_fst(&fst));
}
