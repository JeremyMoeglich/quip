use std::io::Write;

use crate::ast::Operator;

use super::state::Value;

fn join(args: Vec<Value>) -> Value {
    let converted = args
        .iter()
        .map(|x| x.access_property("to_string").try_call(vec![]))
        .reduce(|x, y| x.operation(&Operator::Add, &y))
        .unwrap_or(Value::String("".to_string()));
    converted
}

fn print(args: Vec<Value>) -> Value {
    let joined = join(args);
    if let Value::String(string) = joined {
        print!("{}", string);
        Value::None
    } else if let Value::Error(error) = joined {
        Value::Error(error)
    } else {
        unreachable!("Internal error: join() returned a non-string");
    }
}

fn println(args: Vec<Value>) -> Value {
    let joined = join(args);
    if let Value::String(string) = joined {
        println!("{}", string);
        Value::None
    } else if let Value::Error(error) = joined {
        Value::Error(error)
    } else {
        unreachable!("Internal error: join() returned a non-string");
    }
}

fn input(args: Vec<Value>) -> Value {
    let joined = join(args);
    if let Value::String(string) = joined {
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
        Value::String(input)
    } else if let Value::Error(error) = joined {
        Value::Error(error)
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
