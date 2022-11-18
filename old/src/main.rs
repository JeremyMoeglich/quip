use lang_impl::interpret_code;

fn main() {
    let test_content = include_str!("./example_files/types.qp");


    match interpret_code(test_content, vec![]) {
        Ok(block) => println!("{:#?}", block),
        Err(error) => println!("{}", error),
    }
}