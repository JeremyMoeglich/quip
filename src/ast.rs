lalrpop_mod!(pub parser);
use num::bigint::BigInt;

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    Assignment(TypedIdentifier, Expression),
    Function(Identifier, Vec<TypedIdentifier>, Box<CodeBlock>),
    If(Vec<(Expression, Box<CodeBlock>)>, Option<Box<CodeBlock>>),
    While(Expression, Box<CodeBlock>),
    For(TypedIdentifier, Expression, Box<CodeBlock>),
    Scope(Box<CodeBlock>),
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Range(Box<Section>, Box<Section>),
    Operation(Operation),
}

pub enum Operation {
    Section(Section),
    Operation(Box<Operation>, Operator, Box<Operation>),
}

pub enum Section {
    Unwrap(Box<Expression>),
    Panic(Box<Expression>),
    Range(Box<Expression>, Box<Expression>),
    Spread(Box<Expression>),
    Call(Box<Expression>, Vec<Box<Expression>>),
    Variable(Identifier),
}

#[derive(Debug)]
pub enum Literal {
    Integer(BigInt),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug)]
pub enum Operator {
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

pub type TypedIdentifier = (Identifier, Type);
pub type Type = String;
pub type Identifier = String;
pub type CodeBlock = Vec<Statement>;

pub fn astgen(content: &str) {
    let x = parser::LiteralParser::new().parse(content).unwrap();
    println!("{:?}", x);
}
