use lang_impl::interpret_code;

fn main() {
    let test_content = include_str!("./example_files/types.qp");


    match interpret_code(test_content, vec![]) {
        Ok(block) => println!("{:#?}", block),
        Err(error) => println!("{}", error),
    }
}


use std::convert::TryInto;

fn something<'a, T>(slice: &'a [T]) -> Option<&'a [T; 5]> {
    slice.try_into().ok()
}

use std::convert::TryInto;

fn something<'a, T, const N: usize>(slice: &'a [T]) -> Option<&'a [T; N]> {
    slice.try_into().ok()
}