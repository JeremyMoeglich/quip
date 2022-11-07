use num::bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Return(Expression),
    Assignment(Expression, Expression),
    Declaration(TypedIdentifier, Mutable, Option<Expression>),
    Function(
        Identifier,
        Vec<TypeGeneric>,
        Vec<TypedIdentifier>,
        TypeExpression,
        CodeBlock,
    ),
    If(Expression, CodeBlock, CodeBlock),
    While(Expression, CodeBlock),
    For(TypedIdentifier, Expression, CodeBlock),
    Scope(CodeBlock),
    Expression(Expression),
    StopReturn(Box<Statement>),
    Struct(Identifier, Vec<TypeGeneric>, Vec<TypedIdentifier>),
    Enum(Identifier, Vec<TypeGeneric>, Vec<EnumOption>),
    Impl(
        Identifier,
        Option<Identifier>,
        //Vec<TypeGeneric>, // TODO: Implement generics for impls
        Vec<Statement>,
    ),
    TypeAlias(Identifier, Vec<TypeGeneric>, TypeExpression),
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

pub type TypedIdentifier = (Identifier, TypeExpression);
pub type TypeGeneric = (String, TypeExpression);
pub type Identifier = String;
pub type CodeBlock = Vec<Statement>;
pub type Mutable = bool;
pub type TypeObject = Vec<(Identifier, TypeExpression)>;
pub type EnumOption = (Identifier, Vec<TypeExpression>);

// Types

#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpression {
    Variable(Identifier),
    Array(Box<TypeExpression>),
    Union(Vec<TypeExpression>),
    Object(TypeObject),
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
