use enum_kinds::EnumKind;
use logos::{internal::LexerInternal, Lexer, Logos};
use num::{BigInt, Num};
use std::str::FromStr;

use ast::Number;

#[derive(Logos, Debug, PartialEq, Clone, EnumKind)]
#[enum_kind(TokenKind)]
pub enum Token<'a> {
    // Identifiers
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident(&'a str),

    // literals
    #[regex(
        r"0[xX][0-9a-fA-F]+|0[bB][01]+|0[oO][0-7]+|(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?",
        number
    )]
    Number(Number),

    // strings can use ' or "
    #[regex(r#""([^"\\]|\\.)*""#)]
    DoubleQuoteString(&'a str),
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    SingleQuoteString(&'a str),
    #[regex("r#", raw_string_start)]
    RawString(&'a str),

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
    Coalesce,
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
    #[token("|")]
    VerticalBar,
    #[token("&")]
    Ampersand,

    // Keywords
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("return")]
    Return,
    #[token("true|false", |lex| {
        if lex.slice() == "true" {
            true
        } else {
            false
        }
    })]
    Boolean(bool),

    // Comments
    #[regex(r"//.*")]
    LineComment(&'a str),
    #[regex(r"/\*", block_comment)]
    BlockComment(&'a str),

    // Whitespace
    #[regex(r"( |\n|\t)*")]
    Space(&'a str),

    Error,
}

impl TokenKind {
    pub fn len(&self) -> Option<usize> {
        match &self {
            TokenKind::Range => Some(2),
            TokenKind::And => Some(2),
            TokenKind::Or => Some(2),
            TokenKind::Equal => Some(2),
            TokenKind::NotEqual => Some(2),
            TokenKind::LessThan => Some(1),
            TokenKind::LessThanOrEqual => Some(2),
            TokenKind::GreaterThan => Some(1),
            TokenKind::GreaterThanOrEqual => Some(2),
            TokenKind::Coalesce => Some(2),
            TokenKind::Divide => Some(1),
            TokenKind::Modulo => Some(1),
            TokenKind::Power => Some(2),
            TokenKind::Plus => Some(1),
            TokenKind::Minus => Some(1),
            TokenKind::Star => Some(1),
            TokenKind::Exclamation => Some(1),
            TokenKind::Question => Some(1),
            TokenKind::Dot => Some(1),
            TokenKind::Comma => Some(1),
            TokenKind::Colon => Some(1),
            TokenKind::Semicolon => Some(1),
            TokenKind::LeftParen => Some(1),
            TokenKind::RightParen => Some(1),
            TokenKind::LeftBracket => Some(1),
            TokenKind::RightBracket => Some(1),
            TokenKind::LeftBrace => Some(1),
            TokenKind::RightBrace => Some(1),
            TokenKind::Assign => Some(1),
            TokenKind::ThinArrow => Some(2),
            TokenKind::Let => Some(3),
            TokenKind::If => Some(2),
            TokenKind::Else => Some(4),
            TokenKind::While => Some(5),
            TokenKind::For => Some(3),
            TokenKind::In => Some(2),
            TokenKind::Break => Some(5),
            TokenKind::Continue => Some(8),
            TokenKind::Return => Some(6),
            _ => None,
        }
    }
}

fn number<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Number {
    // 0[xX][0-9a-fA-F]+|0[bB][01]+|0[oO][0-7]+|(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?
    let text = lex.slice();
    // Check if the string starts with 0, indicating it could be a hex, octal, or binary number
    if text.starts_with("0") && text.len() > 1 {
        match text.chars().nth(1).unwrap() {
            'x' | 'X' => {
                return Number::Integer(
                    BigInt::from_str_radix(&text[2..], 16).expect("Invalid hexadecimal number"),
                )
            }
            'o' | 'O' => {
                return Number::Integer(
                    BigInt::from_str_radix(&text[2..], 8).expect("Invalid octal number"),
                )
            }
            'b' | 'B' => {
                return Number::Integer(
                    BigInt::from_str_radix(&text[2..], 2).expect("Invalid binary number"),
                )
            }
            _ => {} // if the second character is not x, o, or b, fall through to parsing as float
        };
    }

    // If not starting with 0x, 0o, or 0b, try parsing as float
    Number::Float(f64::from_str(text).expect("Invalid float number"))
}

impl<'a> Token<'a> {
    pub fn kind(&self) -> TokenKind {
        TokenKind::from(self)
    }
}

fn block_comment<'a>(lex: &mut Lexer<'a, Token<'a>>) -> &'a str {
    let mut depth = 1;
    let start = lex.span().start;

    while depth > 0 {
        match lex.slice().chars().next() {
            Some('/') if lex.slice().starts_with("/*") => {
                lex.bump(2);
                depth += 1;
            }
            Some('*') if lex.slice().starts_with("*/") => {
                lex.bump(2);
                depth -= 1;
            }
            Some(_) => {
                lex.bump(1);
            }
            None => {
                lex.error();
                break;
            }
        }
    }

    &lex.source()[start..lex.span().start]
}

fn raw_string_start<'a>(lex: &mut Lexer<'a, Token<'a>>) -> &'a str {
    let start_hashes = lex.slice().chars().take_while(|&c| c == '#').count();
    let mut end = start_hashes;
    lex.bump(start_hashes);
    while end > 0 {
        match lex.slice().chars().next() {
            Some('#') if lex.slice().ends_with(&"#".repeat(end)) => {
                lex.bump(end);
                end -= 1;
            }
            Some(_) => {
                lex.bump(1);
            }
            None => {
                lex.error();
                break;
            }
        }
    }
    &lex.source()[lex.span().start..lex.span().start + end]
}
