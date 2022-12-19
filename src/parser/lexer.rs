use enum_kinds::EnumKind;
use logos::{Logos, Span};

#[derive(Logos, Debug, PartialEq, EnumKind, Clone)]
#[enum_kind(TokenKind, derive(Hash))]
pub enum Token {
    #[token("fn")]
    Fn,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("extern")]
    Extern,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),
    #[regex("[0-9]+", |lex| lex.slice().to_string())]
    Number(String),

    #[token(";")]
    Semi,

    #[regex(r"([ \t\n\f])+", |lex| lex.slice().to_string())]
    Whitespace(String),
    #[regex(r"//.*(\n|)", |lex| lex.slice().to_string())]
    SingleLineComment(String),
    #[regex(r"/\*(.|\n)*\*/", |lex| lex.slice().to_string())]
    MultiLineComment(String),

    #[error]
    Error,
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        TokenKind::from(self)
    }
    pub fn string(&self) -> String {
        match self {
            Token::Ident(ident) => ident.to_string(),
            Token::Whitespace(whitespace) => whitespace.to_string(),
            Token::SingleLineComment(comment) => comment.to_string(),
            Token::MultiLineComment(comment) => comment.to_string(),
            Token::Number(number) => number.to_string(),
            _ => panic!("Expected string, got {:?}", self),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LocatedToken {
    pub token: Token,
    pub span: Span,
}

impl LocatedToken {
    pub fn new(token: Token, span: Span) -> Self {
        Self { token, span }
    }

    pub fn kind(&self) -> TokenKind {
        self.token.kind()
    }

    pub fn string(&self) -> String {
        self.token.string()
    }
}

pub fn lex(code: &str) -> Vec<LocatedToken> {
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(code);
    while let Some(token) = lexer.next() {
        tokens.push(LocatedToken::new(token, lexer.span()));
    }
    tokens
}
