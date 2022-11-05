mod ast;
mod interpreter;
mod parser;
mod tests;

fn main() {
    let test_content = include_str!("./example_files/2.qp");

    match interpreter::interpret_code(test_content, vec![]) {
        Ok(_state) => (),
        Err(error) => {
            println!("{}", error);
        }
    }
}
