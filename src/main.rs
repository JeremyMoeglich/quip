use quip::interpret_code;


fn main() {
    let test_content = include_str!("../example_files/1.qp");


    match interpret_code(test_content, vec![]) {
        Ok(block) =>  {},
        Err(error) => println!("{}", error),
    }
}