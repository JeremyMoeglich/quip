use quip::parser::parse;


fn main() {
    let code = include_str!("./example_code/1.kld");
    let fst = parse(code);
    println!("{:#?}", fst);
}
