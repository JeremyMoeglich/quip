use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use num::bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub start: Location,
    pub end: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementInner {
    Return(
        // return
        Expression,
    ),
    Assignment(
        Expression,
        // =
        Expression,
    ),
    Declaration(
        // let
        Mutable,
        TypedIdentifier,
        Option<Expression>,
    ),
    Function(
        // fn
        Identifier,
        TypeGenerics,
        TypeList,
        TypeExpression,
        CodeBlock,
    ),
    If(
        // if
        Expression,
        CodeBlock,
        CodeBlock,
    ),
    While(Expression, CodeBlock),
    For(TypedIdentifier, Expression, CodeBlock),
    Scope(CodeBlock),
    Expression(Expression),
    Struct(Identifier, TypeGenerics, Vec<TypedIdentifier>),
    Enum(Identifier, Vec<TypeGeneric>, Vec<EnumOption>),
    Impl(Identifier, Option<Identifier>, TypeGenerics, Vec<Statement>),
    TypeAlias(Identifier, Vec<TypeGeneric>, TypeExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub inner: StatementInner,
    pub location: SourceLocation,
    pub whitespace_after: Whitespace,
    pub returned: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionInner {
    Literal(Literal),
    FancyString(FancyString),
    SingleOperation(SingleOperation, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Get(Box<Expression>, Box<Expression>),
    Variable(Identifier),
    Operation(Box<Expression>, Operator, Box<Expression>),
    List(Vec<Expression>),
    Block(CodeBlock),
    Object(Option<Identifier>, Vec<(Identifier, Expression)>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub inner: ExpressionInner,
    pub location: SourceLocation,
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
    String(String),
    Boolean(bool),
    None,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Integer(int) => write!(f, "{}", int),
            Literal::Float(float) => write!(f, "{}", float),
            Literal::String(string) => write!(f, "{}", string),
            Literal::Boolean(boolean) => write!(f, "{}", boolean),
            Literal::None => write!(f, "None"),
        }
    }
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
    Pipe,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedIdentifier {
    pub identifier: Identifier,
    pub type_expression: TypeExpression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeGeneric {
    pub identifier: Identifier,
    pub type_expression: TypeExpression,
}

pub type TypeGenerics = Vec<TypeGeneric>;

pub type Identifier = String;

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    pub whitespace_before: Whitespace,
    pub statements: Vec<Statement>,
}

pub type Mutable = bool;
pub type TypeList = Vec<TypedIdentifier>;
pub type EnumOption = (Identifier, HashMap<String, TypeExpression>);

// Types

#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpression {
    Variable(Identifier),
    Array(Box<TypeExpression>),
    Union(Vec<TypeExpression>),
    Object(TypeList),
    Intersection(Vec<TypeExpression>),
    Tuple(Vec<TypeExpression>), // TODO: Check if this is necessary
    Infer,
    Empty,
    TypeLiteral(TypeLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeLiteral {
    Integer(BigInt),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
}

// Whitespace

#[derive(Debug, Clone, PartialEq)]
pub enum Space {
    LineComment(String),
    BlockComment(String),
    Blank(usize),
}

pub type Whitespace = Vec<Space>;
