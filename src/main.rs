use parser::simple_parse;


fn main() {
    let test_content = std::fs::read_to_string("example_files/4.qp").expect("Failed to read file");

    match simple_parse(&test_content) {
        Ok(block) =>  {
            println!("{:?}", block);
        },
        Err(error) => println!("{}", error),
    }
}