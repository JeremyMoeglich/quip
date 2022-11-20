use enum_kinds::EnumKind;
use logos::{Logos, Span};
use nom::InputLength;

#[derive(Logos, Debug, PartialEq, EnumKind, Clone)]
#[enum_kind(TokenKind)]
pub enum Token<'a> {
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

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident(&'a str),
    #[regex("[0-9]+")]
    Number(&'a str),

    #[token(";")]
    Semi,

    #[regex(r"([ \t\n\f])+")]
    Whitespace(&'a str),
    #[regex(r"//.*(\n|)")]
    SingleLineComment(&'a str),
    #[regex(r"/\*(.|\n)*\*/")]
    MultiLineComment(&'a str),

    #[error]
    Error,
}

impl Token<'_> {
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
pub struct LocatedToken<'a> {
    pub token: Token<'a>,
    pub span: Span,
}

impl<'a> LocatedToken<'a> {
    pub fn new(token: Token<'a>, span: Span) -> Self {
        Self { token, span }
    }

    pub fn kind(&self) -> TokenKind {
        self.token.kind()
    }

    pub fn string(&self) -> String {
        self.token.string()
    }
}

impl InputLength for LocatedToken<'_> {
    fn input_len(&self) -> usize {
        self.span.end - self.span.start
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
