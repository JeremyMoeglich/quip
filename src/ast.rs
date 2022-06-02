use crate::pest_parser::{ASTParser, Rule};
use num::bigint::BigInt;
use pest::Parser;

enum Statement {
    Return(Expression),
    Assignment(TypedIdentifier, Expression),
    Function(Identifier, Vec<TypedIdentifier>, Box<CodeBlock>),
    If(Vec<(Expression, Box<CodeBlock>)>, Option<Box<CodeBlock>>),
    While(Expression, Box<CodeBlock>),
    For(TypedIdentifier, Expression, Box<CodeBlock>),
    Scope(Box<CodeBlock>),
}

enum Expression {
    Variable(Identifier),
    Literal(Literal),
    Operation(Operator, Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Box<Expression>>),
    Unwrap(Box<Expression>),
    Panic(Box<Expression>),
    Range(Box<Expression>, Box<Expression>),
    Spread(Box<Expression>),
}

enum Literal {
    Integer(BigInt),
    Float(f64),
    String(String),
    Boolean(bool),
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
    Not,
}

type TypedIdentifier = (Identifier, Type);
type Type = String;
type Identifier = String;

pub type CodeBlock = Vec<Statement>;

pub fn create_ast_from_content(content: &str) -> CodeBlock {
    let ast = ASTParser::parse(Rule::code_block, content).unwrap_or_else(|e| panic!("{}", e));

    ast
}
