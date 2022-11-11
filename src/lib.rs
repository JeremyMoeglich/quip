use interpreter::{
    interpret_ast,
    state::{program_state::ProgramState, value::value::Value, value_ref::ValueRef},
};
use parser::simple_parse;

pub mod analysis;
pub mod interpreter;
pub mod parser;

mod tests;

pub fn interpret_code(code: &str, args: Vec<String>) -> Result<(ProgramState, ValueRef), String> {
    match simple_parse(code) {
        Ok(block) => match interpret_ast(block) {
            Ok(state) => match state.get_variable("main") {
                Some(value_ref) => match &*value_ref.resolve().get() {
                    Value::Function(function) => match function.parameters.len() {
                        0 => {
                            let returned = value_ref.try_call(vec![]);
                            Ok((state, returned))
                        }
                        1 => {
                            let returned = value_ref.try_call(
                                args.into_iter()
                                    .map(|arg| ValueRef::new(Value::String(arg)))
                                    .collect(),
                            );
                            Ok((state, returned))
                        }
                        _ => Err(format!(
                            "main function takes 0 or 1 arguments, but {} were given",
                            function.parameters.len()
                        )),
                    },
                    _ => {
                        println!("WARNING: main is not a function");
                        Ok((state, ValueRef::none()))
                    }
                },
                _ => Ok((state, ValueRef::none())),
            },
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    }
}
