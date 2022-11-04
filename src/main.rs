use crate::parser::simple_parse;

mod ast;
mod interpreter;
mod parser;
mod tests;

fn main() {
    let test_content = r#""5".to_int() + 5"#;

    println!("{:#?}", simple_parse(test_content));

    // match interpreter::interpret_code(test_content, vec![]) {
    //     Ok(_state) => (),
    //     Err(error) => {
    //         println!("{}", error);
    //     }
    // }
}
