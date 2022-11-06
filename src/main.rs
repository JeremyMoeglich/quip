use lang_impl::parser;

fn main() {
    let test_content = include_str!("./example_files/types.qp");

    match parser::simple_parse(test_content) {
        Ok(block) => println!("{:#?}", block),
        Err(error) => println!("{}", error),
    }
}
