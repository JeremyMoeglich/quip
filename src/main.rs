mod ast;
mod interpreter;
mod parser;

fn main() {
    let test_content = r###"
let a = 1;

fn main() {
    a = a - 4;
    let b = 2;
    let c = a + b;
    if c == 3 {
        "Hello, world!"
    } else if c == 4 {
        b
    } else {
        c
    }
}
    "###;
    let expr = parser::simple_parse(test_content);
    println!("{:#?}", expr);
    let state = interpreter::interpret_ast(expr.unwrap());
    println!("Final State {:#?}", state);
    println!("{:#?}", state.borrow().run_function("main", vec![]));
    println!("{:#?}", state.borrow().run_function("main", vec![]));
}
