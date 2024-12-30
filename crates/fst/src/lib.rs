use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

use num::bigint::BigInt;
use vec1::Vec1;

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
        write!(f, "line:{} - col:{}", self.line + 1, self.column)
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
/// This represents a span within a source file
/// The range includes the start, but excludes the end
pub struct SourceSpan {
    pub start: Location,
    pub end: Location,
}

impl Display for SourceSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LabelExpression {
    WithExpression {
        pre_space: Whitespace1,
        label: Option<(String, Whitespace1)>,
        expr: Expression,
        // expressions at the end of a statement must end with a semicolon
        semi_space: Whitespace0,
    },
    NoExpression(SpacedLabel),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpacedLabel {
    label: Option<(Whitespace1, String)>,
    semi_space: Option<Whitespace0>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression {
        // - {expr}
        // - {expr}{space0};
        expr: Expression,
        semi: Option<Whitespace0>,
    },
    // - return{label_expression}
    Return(LabelExpression),
    // - break{label_expression}
    Break(LabelExpression),
    // - continue{spaced_label}
    Continue(SpacedLabel),
    Function {
        name: String,
        closure: Closure,
    },
    Struct {
        name: String,
        fields: Vec<(String, Expression)>,
    },
    Enum {
        name: String,
        options: Vec<EnumOption>,
    },
    Trait {
        name: String,
        signatures: Vec<Signature>,
    },
    Impl {
        target: String,
        implemented: Option<Expression>,
        statements: Vec<Statement>,
    },
    Import {
        importable: Expression,
        extract: Option<MutableExtract>,
    },
    Module {
        name: String,
        statements: Vec<Statement>,
    },
    Env(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub closure_signature: ClosureSignature,
    pub body: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClosureSignature {
    pub params: Vec<(VariableCreation, Option<Expression>)>,
    pub return_type: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub after_fn: Whitespace1,
    pub name: String,
    pub after_name: Whitespace0,
    pub closure_signature: ClosureSignature,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Signature {
    Function(FunctionSignature),
    Property(PropertySignature),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertySignature {
    pub mutable: bool,
    pub name: String,
    pub value_type: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CallArguments {
    Named(Vec<(String, Expression)>),
    Positional(Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal {
        value: Literal,
    },
    Variable {
        identifier: String,
    },
    SingleOperation {
        operation: UnaryOperation,
        operand: Box<Expression>,
    },
    /// a + b
    Operation {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    /// [1, 2, 3]
    Array {
        elements: Vec<Expression>,
    },
    /// let x = 1; let mut x = 1;
    ///
    /// This is an expression, this is useful when declaring within a control structure
    /// if ((let x = 1) == 5) {
    ///     do_something();
    /// }
    Declaration {
        creation: VariableCreation,
        value_type: Option<Box<Expression>>,
        initializer: Option<Box<Expression>>,
    },
    /// let f = x -> x + 1;
    /// let f = (x, y) -> x + y;
    /// let f = (x: U64, y: U64) -> U64 do { x + y };
    /// let f = x -> U64 do x + 1;
    Closure
    /// { stat1; stat2; stat3 }
    /// scope_expr: { stat1; stat2; stat3 }
    Block {
        environment: Option<Box<Expression>>,
        block: Vec<Statement>,
    },
    If {
        blocks: Vec<(Expression, Vec<Statement>)>, // multiple blocks occur when using the `else if` syntax
        else_block: Option<Vec<Statement>>,
    },
    While {
        label: Option<String>,
        condition: Box<Expression>,
        body: Box<Expression>,
        else_block: Option<Box<Expression>>,
    },
    Loop {
        label: Option<String>,
        body: Box<Expression>,
    },
    For {
        label: Option<String>,
        destructure: MutableDestructure,
        iterator: Box<Expression>,
        body: Box<Expression>,
        else_block: Option<Box<Expression>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableCreation {
    Identifier { name: String, mutable: bool },
    Destructure { destructure: MutableDestructure },
}

/// This is a destructure
///
/// Examples:
/// { mut a, b }
/// { a as mut c, b as d }
/// { mut a.b }
/// { a.{b, mut c} }
/// { a.{b.f as e, c} as mut d }
///
pub type MutableDestructure = Vec<MutableDestructureProperty>;
pub type ImmutableDestructure = Vec<ImmutableDestructureProperty>;

#[derive(Debug, Clone, PartialEq)]
pub enum MutableDestructureProperty {
    AliasedSubProperties {
        property_name: String,
        extract: ImmutableExtract,
        alias: MutableAlias,
    },
    Property {
        property_name: String,
        alias: Option<MutableAlias>,
    },
    UnaliasedSubProperties {
        property_name: String,
        extract: MutableExtract,
    },
    MutablePropertyChain {
        property_chain: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MutableAlias {
    pub mutable: bool,
    pub alias: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImmutableDestructureProperty {
    pub property_name: String,
    pub extract: Option<ImmutableExtract>,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MutableExtract {
    Destructured(MutableDestructure),
    DirectProperty(Box<MutableDestructureProperty>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImmutableExtract {
    Destructured(ImmutableDestructure),
    DirectProperty(Box<ImmutableDestructureProperty>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperation {
    Not,
    ErrorUnwrap,
    Inline,
    Spread,
    Negate,
    Positive,
    Reference {
        mutable: bool,
    },
    Dereference,
    /// f(a, b)
    /// f { a, b }
    Call {
        arguments: CallArguments,
    },
    /// a[b]
    Get {
        property: Box<Expression>,
    },
    /// a.{b, c}
    /// a.{b as c, d as e}
    /// a.{b}
    /// a.b
    Extract {
        extract: ImmutableExtract,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(String),
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
    Assignment,          // =
    Range,               // 1..10
    And,                 // true && false
    Or,                  // true || false
    Equals,              // 1 == 1, "a" == "a"
    NotEquals,           // 1 != 1, "a" != "a"
    LessThan,            // 1 < 2, "a" < "b"
    LessThanOrEquals,    // 1 <= 2, "a" <= "b"
    GreaterThan,         // 1 > 2, "a" > "b"
    GreaterThanOrEquals, // 1 >= 2, "a" >= "b"
    Add,                 // 1 + 2, "a" + "b"
    Subtract,            // 1 - 2, "a" - "b"
    Multiply,            // 1 * 2, "a" * "b"
    WrappingAdd,         // 1 +% 2, "a" +% "b"
    WrappingSubtract,    // 1 -% 2, "a" -% "b"
    WrappingMultiply,    // 1 *% 2, "a" *% "b"
    Divide,              // 1 / 2, "a" / "b"
    Modulo,              // 1 % 2, "a" % "b"
    Power,               // 1 ** 2, "a" ** "b"
    Pipe,                // 1 |> 2, "a" |> "b"
    Union,               // 1 | 2, "a" | "b"
    Intersection,        // 1 & 2, "a" & "b"
    ExclusiveOr,         // 1 ^ 2, "a" ^ "b"
}

pub type Mutable = bool;
pub type EnumOption = (String, EnumValue);

#[derive(Debug, Clone, PartialEq)]
pub enum EnumValue {
    Tuple(Vec<Expression>),
    Struct(Vec<(String, Expression)>),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeLiteral {
    Number(Number),
    String(String),
    Boolean(bool),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpaceElement {
    LineComment(String),
    BlockComment(String),
    Space(String),
}

pub type Whitespace0 = Vec<SpaceElement>;
pub type Whitespace1 = Vec1<SpaceElement>;

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub pre_space: Whitespace0,
    pub statements: Vec<(Statement, Whitespace0)>,
}
