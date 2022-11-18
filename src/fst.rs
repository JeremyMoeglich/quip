#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Extern {
        space_extern_ident: Space,
        name: Ident,
        space_ident_lparen: Space,
        space_lparen_arg1: Space,
        args: Arguments,
        right_space: Space,
    },
    ImplicitReturn {
        value: Expression,
    },
    Function {
        space_fn_ident: Space,
        name: Ident,
        space_ident_lparen: Space,
        space_lparen_arg1: Space,
        args: Arguments,
        space_rparen_lbrace: Space,
        space_lbrace_expr: Space,
        body: CodeBlock,
        right_space: Space,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpacePart {
    Whitespace(String),
    SingleLineComment(String),
    MultiLineComment(String),
}

pub type Space = Vec<SpacePart>;
pub type Ident = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Call {
        name: Ident,
        space_ident_lparen: Space,
        space_lparen_arg1: Space,
        params: Parameters,
        right_space: Space,
    },
    Segment {
        segment: Segment,
    },
}

pub type Arguments = Vec<Argument>;

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: Ident,
    pub space_ident_right: Space,
    pub space_after_comma: Option<Space>,
}

impl Argument {
    pub fn new(name: String, space_ident_right: Space, space_after_comma: Option<Space>) -> Self {
        Argument {
            name,
            space_ident_right,
            space_after_comma,
        }
    }
}

pub type Parameters = Vec<Parameter>;

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub expression: Expression,
    pub space_ident_right: Space,
    pub space_after_comma: Option<Space>,
}

impl Parameter {
    pub fn new(
        expression: Expression,
        space_ident_right: Space,
        space_after_comma: Option<Space>,
    ) -> Self {
        Parameter {
            expression,
            space_ident_right,
            space_after_comma,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Number { number: f64, right_space: Space },
    Ident { ident: String, right_space: Space },
}

pub type CodeBlock = Vec<Statement>;

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
