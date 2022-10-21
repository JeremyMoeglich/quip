mod ast;
mod parser;

fn main() {
    let test_content = r#"((**))"#;
    let expr = parser::simple_parse(test_content);
    println!("{:#?}", expr);
}
