mod access;
mod code_block;
mod expression;
mod native_functions;
pub mod state;
mod statement;

use crate::ast::CodeBlock;

use self::{
    code_block::interpret_code_block,
    native_functions::NATIVE_FUNCTIONS,
    state::{program_state::ProgramState, value::value::Value, value_ref::ValueRef},
};

pub fn interpret_ast(block: CodeBlock) -> Result<ProgramState, String> {
    let mut state = ProgramState::new();
    let native_functions = NATIVE_FUNCTIONS
        .iter()
        .map(|(name, value)| (name.to_string(), value))
        .collect::<Vec<_>>();
    let res = interpret_code_block(
        &block,
        &state,
        native_functions
            .iter()
            .map(|(name, value)| (name, ValueRef::new((*value).clone())))
            .collect::<Vec<_>>(),
    );
    state = res.1;
    if let Value::Error(error) = &*res.0.clone().get() {
        Err(error.clone())
    } else {
        Ok(state)
    }
}

pub fn interpret_code(code: &str, args: Vec<String>) -> Result<(ProgramState, ValueRef), String> {
    match crate::parser::simple_parse(code) {
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
        Err(error) => Err(error.to_string()),
    }
}
