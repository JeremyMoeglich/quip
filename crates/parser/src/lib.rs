#![feature(return_position_impl_trait_in_trait)]
#![feature(closure_lifetime_binder)]

mod block;
mod error;
pub mod expression;
mod identifier;
mod statement;
mod utils;

use self::{statement::parse_statement, utils::ws0};
use ast::CodeBlock;
use error::create_fancy_error;
use parser_core::*;

pub mod core {
    pub use parser_core::*;
}

pub fn parse_code<'a>(input: &Span<'a>) -> ParserResult<'a, CodeBlock> {
    let (input, out) = many0(parse_statement)(input)?;
    let (input, _) = ws0(&input)?;
    Ok((input, CodeBlock { statements: out }))
}

pub fn simple_parse(code: &str) -> Result<CodeBlock, String> {
    let tokens = tokenize(code);
    let input = create_span(&tokens);
    let result = parse_code(&input);
    match result {
        Ok((input2, expression)) => match input2.tokens.len() {
            0 => Ok(expression),
            _ => Err(create_fancy_error(
                &code,
                LocatedParserError::new(
                    ParserError::UnexpectedToken(input2.tokens[0].kind(), vec![TokenKind::EOF]),
                    input2.start,
                ),
            )),
        },
        Err(err) => Err(create_fancy_error(&code, err)),
    }
}
