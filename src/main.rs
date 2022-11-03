mod ast;
mod interpreter;
mod parser;

fn main() {
    let test_content = r###"
fn main() {
    let a = /* 1 + 2 */ 3;
}
    "###;
    let expr = parser::simple_parse(test_content);
    let state = interpreter::interpret_ast(expr.unwrap());
    println!("{:?}", state.run_function("main", vec![]));
}
