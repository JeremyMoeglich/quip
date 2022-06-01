use std::str::FromStr;

use logos::{Lexer, Logos};
use num::BigInt;

fn str_token(lex: &mut Lexer<Token>) -> String {
    lex.slice().to_string()
}

fn int_token(lex: &mut Lexer<Token>) -> Option<BigInt> {
    Some(BigInt::from_str(lex.slice()).ok()?)
}

fn float_token(lex: &mut Lexer<Token>) -> Option<f64> {
    Some(f64::from_str(lex.slice()).ok()?)
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[\r\t\f\v ]+", logos::skip)]
    #[error]
    Error,
    #[token("\n")]
    Newline,
    #[token("fn")]
    Function,
    #[token("mut")]
    Mut,
    #[regex("(\"[^\"]*\")|('[^']*')", priority = 3, callback = str_token)]
    String(String),
    #[regex("[0-9]+", callback = int_token)]
    Integer(BigInt),
    #[regex("[0-9]*\\.[0-9]+", callback = float_token)]
    Float(f64),
    #[token("=")]
    Assign,
    #[token(";")]
    Semicolon,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", callback = str_token)]
    Identifier(String),
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Exclamation,
    #[token("?")]
    Question,
    #[token("!?")]
    Panic,
    #[token("..")]
    Range,
    #[token("...")]
    Spread,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[regex("//[^\n]*", priority = 2)]
    Comment,
    #[token("|>")]
    Pipe,
}
