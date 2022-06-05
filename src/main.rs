#[macro_use] extern crate lalrpop_util;

mod ast;

fn main() {
    let test_content = "(24444444444444444444444)";
    ast::astgen(test_content);
}
