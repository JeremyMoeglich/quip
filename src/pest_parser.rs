extern crate pest;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ASTParser;

