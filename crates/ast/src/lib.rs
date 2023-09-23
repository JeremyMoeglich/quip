use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Display, Formatter},
};

use num::bigint::BigInt;

#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "line:{} - col:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub start: Location,
    pub end: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementInner {
    Return {
        expr: Expression,
    },
    Assignment {
        left: Expression,
        right: Expression,
    },
    Declaration {
        mutable: Mutable,
        identifier: TypedIdentifier,
        initializer: Option<Expression>,
    },
    Function {
        name: Identifier,
        generics: TypeGenerics,
        params: TypeList,
        ret_type: Expression,
        body: CodeBlock,
    },
    If {
        condition: Expression,
        then_block: CodeBlock,
        else_block: CodeBlock,
    },
    While {
        condition: Expression,
        body: CodeBlock,
    },
    For {
        variable: TypedIdentifier,
        range: Expression,
        body: CodeBlock,
    },
    Scope {
        body: CodeBlock,
    },
    Expression {
        expr: Expression,
    },
    Struct {
        name: Identifier,
        generics: TypeGenerics,
        fields: Vec<TypedIdentifier>,
    },
    Enum {
        name: Identifier,
        type_generics: Vec<TypeGeneric>,
        options: Vec<EnumOption>,
    },
    Impl {
        target: Identifier,
        trait_name: Option<Identifier>,
        generics: TypeGenerics,
        statements: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub inner: StatementInner,
    pub returned: bool,
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
    Object(Option<Identifier>, Vec<(Identifier, Expression)>),
    Infer,
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
    Number(Number),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(BigInt),
    Float(f64),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(number) => write!(f, "{}", number),
            Literal::String(string) => write!(f, "{}", string),
            Literal::Boolean(boolean) => write!(f, "{}", boolean),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Integer(integer) => write!(f, "{}", integer),
            Number::Float(float) => write!(f, "{}", float),
        }
    }
}

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
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeGeneric {
    pub identifier: Identifier,
    pub expression: Expression,
}

pub type TypeGenerics = Vec<TypeGeneric>;

pub type Identifier = String;

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    pub statements: Vec<Statement>,
}

pub type Mutable = bool;
pub type TypeList = Vec<TypedIdentifier>;
pub type EnumOption = (Identifier, EnumValue);

#[derive(Debug, Clone, PartialEq)]
pub enum EnumValue {
    Tuple(Vec<Expression>),
    Struct(Vec<TypedIdentifier>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeLiteral {
    Number(Number),
    String(String),
    Boolean(bool),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Comment {
    Line(String),
    Block(String),
}

pub type Whitespace = Vec<Comment>;
