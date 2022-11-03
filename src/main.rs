mod ast;
mod interpreter;
mod parser;

fn main() {
    let test_content = r###"
fn main() {
    let a = 1; // This is a comment
    let b = 2.5;
    println("Hello, {a + b}!");
}
    "###;
    let expr = parser::simple_parse(test_content);
    println!("{:#?}", expr);
    let state = interpreter::interpret_ast(expr.unwrap());
    println!("{:?}", state.run_function("main", vec![]));
}
