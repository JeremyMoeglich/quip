use num::BigInt;

use crate::parser::fst::{Identifier, Operator, SingleOperation, TypeExpression};

#[derive(Debug, Clone, PartialEq)]
pub struct TypedExpression {
    pub expression: TypedExpressionInner,
    pub return_type: TypeExpression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypedExpressionInner {
    Literal(TypedLiteral),
    SingleOperation(SingleOperation, Box<TypedExpression>),
    Call(Box<TypedExpression>, Vec<TypedExpression>),
    Get(Box<TypedExpression>, Box<TypedExpression>),
    Variable(Identifier),
    Operation(Box<TypedExpression>, Operator, Box<TypedExpression>),
    List(Vec<TypedExpression>),
    Block(TypedCodeBlock),
    Object(Option<Identifier>, Vec<(Identifier, TypedExpression)>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypedLiteral {
    Integer(BigInt),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
}
