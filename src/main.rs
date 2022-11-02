mod ast;
mod parser;
mod interpreter;

fn main() {
    let test_content = r###"
fn main() {
    let a = 1;
    let b = 2;
    let c = a + b;
    if c == 3 {
        print("Hello, world!");
    } else if c == 4 {
        print(b);
    } else {
        print("C: {c}");
    }
}    
    "###;
    let expr = parser::simple_parse(test_content);
    println!("{:#?}", expr);
}
