use crate::*;
use enum_kinds::EnumKind;
use enumset::EnumSetType;
use logos::{internal::LexerInternal, Lexer, Logos};
use proc_macros::TokenParser;


#[derive(Logos, Debug, PartialEq, Clone, Copy, EnumKind, TokenParser)]
#[enum_kind(TokenKind, derive(EnumSetType), enumset(no_super_impls))]
pub enum Token<'a> {
    // Identifiers
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident(&'a str),

    // literals
    #[regex(r"0[xX][0-9a-fA-F]+|0[bB][01]+|0[oO][0-7]+|(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?")]
    Number(&'a str),

    // strings can use ' or "
    #[regex(r#""([^"\\]|\\.)*"|'([^'\\]|\\.)*'"#)]
    String(&'a str),
    #[regex("r#", raw_string_start)]
    RawString(&'a str),

    // labels
    #[regex("'[a-zA-Z_][a-zA-Z0-9_]*")]
    Label(&'a str),

    #[token("true|false", |lex| {
        if lex.slice() == "true" {
            true
        } else {
            false
        }
    })]
    Boolean(bool),

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
    #[token("|>")]
    Pipe,
    #[token("^")]
    Caret,

    // operators that can be used as a prefix, postfix, or inbetween
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("+%")]
    PlusPercent,
    #[token("-%")]
    MinusPercent,
    #[token("*%")]
    StarPercent,
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
    Assignment,
    #[token("->")]
    Arrow,
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
    #[token("loop")]
    Loop,
    #[token("in")]
    In,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("return")]
    Return,
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("impl")]
    Impl,
    #[token("trait")]
    Trait,
    #[token("mod")]
    Mod,
    #[token("type")]
    Type,
    #[token("fn")]
    Fn,
    #[token("mut")]
    Mut,
    #[token("import")]
    Import,
    #[token("as")]
    As,
    #[token("do")]
    Do,
    #[token("use_env")]
    UseEnv,

    // Comments
    #[regex(r"//.*")]
    LineComment(&'a str),
    #[regex(r"/\*", block_comment)]
    BlockComment(&'a str),

    // Whitespace
    #[regex("[ \r\n\t]+")]
    Space(&'a str),

    Error,
}

impl TokenKind {
    #[inline]
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
            TokenKind::Assignment => Some(1),
            TokenKind::Arrow => Some(2),
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

    pub fn kind_name(&self) -> &'static str {
        match self {
            TokenKind::Ident => "Identifier",
            TokenKind::Number => "Number",
            TokenKind::String => "String",
            TokenKind::RawString => "RawString",
            TokenKind::Label => "Label",
            TokenKind::Boolean => "Boolean",
            TokenKind::Range => "Range",
            TokenKind::And => "And",
            TokenKind::Or => "Or",
            TokenKind::Equal => "Equal",
            TokenKind::NotEqual => "NotEqual",
            TokenKind::LessThan => "LessThan",
            TokenKind::LessThanOrEqual => "LessThanOrEqual",
            TokenKind::GreaterThan => "GreaterThan",
            TokenKind::GreaterThanOrEqual => "GreaterThanOrEqual",
            TokenKind::Coalesce => "Coalesce",
            TokenKind::Divide => "Divide",
            TokenKind::Modulo => "Modulo",
            TokenKind::Power => "Power",
            TokenKind::Pipe => "Pipe",
            TokenKind::Caret => "Caret",
            TokenKind::Plus => "Plus",
            TokenKind::Minus => "Minus",
            TokenKind::Star => "Star",
            TokenKind::PlusPercent => "PlusPercent",
            TokenKind::MinusPercent => "MinusPercent",
            TokenKind::StarPercent => "StarPercent",
            TokenKind::Exclamation => "Exclamation",
            TokenKind::Question => "Question",
            TokenKind::Dot => "Dot",
            TokenKind::Comma => "Comma",
            TokenKind::Colon => "Colon",
            TokenKind::Semicolon => "Semicolon",
            TokenKind::LeftParen => "LeftParen",
            TokenKind::RightParen => "RightParen",
            TokenKind::LeftBracket => "LeftBracket",
            TokenKind::RightBracket => "RightBracket",
            TokenKind::LeftBrace => "LeftBrace",
            TokenKind::RightBrace => "RightBrace",
            TokenKind::Assignment => "Assignment",
            TokenKind::Arrow => "Arrow",
            TokenKind::VerticalBar => "VerticalBar",
            TokenKind::Ampersand => "Ampersand",
            TokenKind::Let => "Let",
            TokenKind::If => "If",
            TokenKind::Else => "Else",
            TokenKind::While => "While",
            TokenKind::For => "For",
            TokenKind::Loop => "Loop",
            TokenKind::In => "In",
            TokenKind::Break => "Break",
            TokenKind::Continue => "Continue",
            TokenKind::Return => "Return",
            TokenKind::Struct => "Struct",
            TokenKind::Enum => "Enum",
            TokenKind::Impl => "Impl",
            TokenKind::Trait => "Trait",
            TokenKind::Mod => "Mod",
            TokenKind::Type => "Type",
            TokenKind::Fn => "Function",
            TokenKind::Mut => "Mutable",
            TokenKind::Import => "Import",
            TokenKind::As => "As",
            TokenKind::Do => "Do",
            TokenKind::UseEnv => "UseEnv",
            TokenKind::LineComment => "LineComment",
            TokenKind::BlockComment => "BlockComment",
            TokenKind::Space => "Space",
            TokenKind::Error => "Error",
        }
    }
}

// fn number<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Number {
//     let text = lex.slice();
//     let mut is_float = false;

//     // Check for prefixed numbers: hex, octal, or binary
//     if text.starts_with("0") && text.len() > 1 {
//         match text.chars().nth(1).unwrap() {
//             'x' | 'X' => {
//                 return Number::Integer(
//                     BigInt::from_str_radix(&text[2..], 16).expect("Invalid hexadecimal number"),
//                 )
//             }
//             'o' | 'O' => {
//                 return Number::Integer(
//                     BigInt::from_str_radix(&text[2..], 8).expect("Invalid octal number"),
//                 )
//             }
//             'b' | 'B' => {
//                 return Number::Integer(
//                     BigInt::from_str_radix(&text[2..], 2).expect("Invalid binary number"),
//                 )
//             }
//             _ => {}
//         }
//     }

//     // Manually parse the number to determine if it's an integer or float
//     for (i, c) in text.chars().enumerate() {
//         match c {
//             '0'..='9' => continue, // Digits are fine
//             '.' => {
//                 // Only one decimal point allowed
//                 if is_float {
//                     panic!("Invalid float number: multiple decimal points");
//                 }
//                 is_float = true;
//             }
//             'e' | 'E' => {
//                 // Handle scientific notation; if encountered, the rest must be float
//                 is_float = true;
//                 // Check if the exponent is properly formatted
//                 if i + 1 < text.len() {
//                     match text.chars().nth(i + 1).unwrap() {
//                         '+' | '-' => {
//                             // Skip sign, must be followed by digits
//                             if i + 2 >= text.len() || !text.chars().nth(i + 2).unwrap().is_digit(10)
//                             {
//                                 panic!("Invalid float number: malformed exponent");
//                             }
//                         }
//                         d if d.is_digit(10) => continue,
//                         _ => panic!("Invalid float number: malformed exponent"),
//                     }
//                 } else {
//                     panic!("Invalid float number: missing exponent digits");
//                 }
//                 break; // Rest must be digits
//             }
//             _ => panic!("Invalid character in number: '{}'", c), // Non-numeric characters
//         }
//     }

//     // Return the parsed number
//     if is_float {
//         Number::Float(f64::from_str(text).expect("Invalid float number"))
//     } else {
//         Number::Integer(BigInt::from_str(text).expect("Invalid integer number"))
//     }
// }

impl<'a> Token<'a> {
    #[inline]
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
