#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Extern(ExternStatement),
    ImplicitReturn(Expression),
    Function(FunctionStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternStatement {
    pub space_extern_ident: Space,
    pub name: Ident,
    pub space_ident_lparen: Space,
    pub space_lparen_arg1: Space,
    pub params: Parameters,
    pub right_space: Space,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStatement {
    pub space_fn_ident: Space,
    pub name: Ident,
    pub space_ident_lparen: Space,
    pub space_lparen_arg1: Space,
    pub params: Parameters,
    pub space_rparen_lbrace: Space,
    pub body: CodeBlock,
    pub right_space: Space,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpacePart {
    Whitespace(String),
    SingleLineComment(String),
    MultiLineComment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Space {
    pub space: Vec<SpacePart>,
}

impl Space {
    pub fn new(space: Vec<SpacePart>) -> Self {
        Self { space }
    }
    pub fn has_comments(&self) -> bool {
        self.space.iter().any(|space_part| match space_part {
            SpacePart::SingleLineComment(_) => true,
            SpacePart::MultiLineComment(_) => true,
            _ => false,
        })
    }
    pub fn empty() -> Self {
        Space { space: vec![] }
    }
}

impl<'a> IntoIterator for &'a Space {
    type Item = &'a SpacePart;
    type IntoIter = std::slice::Iter<'a, SpacePart>;
    fn into_iter(self) -> Self::IntoIter {
        self.space.iter()
    }
}

pub type Ident = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Call(CallExpression),
    Segment(Segment),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression {
    pub name: Ident,
    pub space_ident_lparen: Space,
    pub space_lparen_arg1: Space,
    pub args: Arguments,
    pub right_space: Space,
}

pub type Arguments = Vec<Argument>;

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub expr: Expression,
    pub space_ident_right: Space,
    pub space_after_comma: Option<Space>,
}

impl Argument {
    pub fn new(
        expr: Expression,
        space_ident_right: Space,
        space_after_comma: Option<Space>,
    ) -> Self {
        Argument {
            expr,
            space_ident_right,
            space_after_comma,
        }
    }
}

pub type Parameters = Vec<Parameter>;

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Ident,
    pub space_ident_right: Space,
    pub space_after_comma: Option<Space>,
}

impl Parameter {
    pub fn new(name: Ident, space_ident_right: Space, space_after_comma: Option<Space>) -> Self {
        Parameter {
            name,
            space_ident_right,
            space_after_comma,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Number(NumberSegment),
    Ident(IdentSegment),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberSegment {
    pub number: String,
    pub right_space: Space,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentSegment {
    pub ident: String,
    pub right_space: Space,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    pub space_brace_stat1: Space,
    pub statements: Vec<Statement>,
    pub right_space: Space,
}

impl CodeBlock {
    pub fn new(space_brace_stat1: Space, statements: Vec<Statement>, right_space: Space) -> Self {
        CodeBlock {
            space_brace_stat1,
            statements,
            right_space,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fst {
    pub beginning_space: Space,
    pub index_block: Vec<Statement>,
}

impl Fst {
    pub fn new(beginning_space: Space, index_block: Vec<Statement>) -> Self {
        Fst {
            beginning_space,
            index_block,
        }
    }
}
