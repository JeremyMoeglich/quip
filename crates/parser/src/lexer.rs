use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
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
    String,
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    Char,
}
