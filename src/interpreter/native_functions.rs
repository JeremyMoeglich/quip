use std::io::Write;

use crate::parser::ast::Operator;

use super::state::{value::value::Value, value_ref::ValueRef};

fn join(args: Vec<ValueRef>) -> ValueRef {
    let converted = args
        .iter()
        .map(|x| x.get_property("to_string").try_call(vec![]))
        .reduce(|x, y| x.operation(&Operator::Add, &y))
        .unwrap_or(ValueRef::new(Value::String("".to_string())));
    converted.resolve()
}

fn print(args: Vec<ValueRef>) -> ValueRef {
    let joined = join(args);
    if let Value::String(string) = &*joined.clone().get() {
        print!("{}", string);
        ValueRef::none()
    } else if let Value::Error(_) = &*joined.get() {
        joined.clone()
    } else {
        unreachable!("Internal error: join() returned a non-string");
    }
}

fn println(args: Vec<ValueRef>) -> ValueRef {
    let joined = join(args);
    if let Value::String(string) = &*joined.clone().get() {
        println!("{}", string);
        ValueRef::none()
    } else if let Value::Error(_) = &*joined.get() {
        joined.clone()
    } else {
        unreachable!("Internal error: join() returned a non-string");
    }
}

fn input(args: Vec<ValueRef>) -> ValueRef {
    let joined = join(args);
    if let Value::String(string) = &*joined.clone().get() {
        let mut input = String::new();
        print!("{}", string);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        ValueRef::new(Value::String(input))
    } else if let Value::Error(_) = &*joined.get() {
        joined.clone()
    } else {
        unreachable!("Internal error: join() returned a non-string");
    }
}

pub const NATIVE_FUNCTIONS: &[(&str, Value)] = &[
    ("join", Value::NativeFunction(vec![], join)),
    ("print", Value::NativeFunction(vec![], print)),
    ("println", Value::NativeFunction(vec![], println)),
    ("input", Value::NativeFunction(vec![], input)),
];
