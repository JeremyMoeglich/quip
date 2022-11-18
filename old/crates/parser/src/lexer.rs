use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Eq)]
pub enum Token {
    // Identifiers
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,

    // literals
    #[regex(r"[0-9]+\.[0-9]+")]
    Float,
    #[regex(r"[0-9]+")]
    Integer,

    // strings can use ' or "
    #[regex(r#""([^"\\]|\\.)*""#)]
    DoubleQuoteString,
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    SingleQuoteString,

    // operators
    #[token("..")]
    Range,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessThanOrEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanOrEqual,
    #[token("??")]
    NullCoalesce,
    #[token("//")]
    IntegerDivide,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("**")]
    Power,

    // operators that can be used as a prefix, postfix, or inbetween
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("!")]
    Exclamation,
    #[token("?")]
    Question,

    // other tokens
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("=")]
    Assign,
    #[token("->")]
    ThinArrow,

}
