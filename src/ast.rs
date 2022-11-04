#![allow(dead_code)]
use num::bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Return(Expression),
    Assignment(Expression, Expression),
    Declaration(TypedIdentifier, Mutable, Option<Expression>),
    Function(Identifier, Vec<TypedIdentifier>, CodeBlock),
    If(Expression, CodeBlock, CodeBlock),
    While(Expression, CodeBlock),
    For(TypedIdentifier, Expression, CodeBlock),
    Scope(CodeBlock),
    Expression(Expression),
    StopReturn(Box<Statement>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    SingleOperation(SingleOperation, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Get(Box<Expression>, Box<Expression>),
    Variable(Identifier),
    Operation(Box<Expression>, Operator, Box<Expression>),
    List(Vec<Expression>),
    Block(CodeBlock),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleOperation {
    Not,
    ErrorUnwrap,
    Panic,
    Spread,
    Negate,
    Positive,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(BigInt),
    Float(f64),
    String(FancyString),
    Boolean(bool),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FancyStringFragment {
    Expression(Expression),
    LiteralString(String),
    FormatPlaceholder,
}

pub type FancyString = Vec<FancyStringFragment>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Range,
    And,
    Or,
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    Coalesce,
    Add,
    Subtract,
    Multiply,
    IntDivide,
    Divide,
    Modulo,
    Power,
    Access,
}

pub type TypedIdentifier = (Identifier, String); // TODO: Type
pub type Identifier = String;
pub type CodeBlock = Vec<Statement>;
pub type Mutable = bool;

// Types

#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpression {
    Variable(Identifier),
    Array(Box<TypeExpression>),
    Union(Vec<TypeExpression>),
}
