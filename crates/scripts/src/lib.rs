use std::collections::VecDeque;

use fst::{Location, SourceSpan};
use enumset::EnumSet;
use parser::{
    core::{create_span, LocatedToken, Token, TokenKind},
    parse_file,
};

pub fn fuzz_possibilities(mut on_found: impl FnMut(&Vec<LocatedToken<'static>>)) {
    let mut queue = VecDeque::new();
    queue.push_back((Vec::new(), interesting_tokens()));
    while let Some((tokens, following_tokens)) = queue.pop_front() {
        for token in iter_tokens(following_tokens) {
            let mut new_tokens = tokens.clone();
            let start_index = new_tokens.len();
            let start_location = Location {
                column: start_index,
                line: 0,
                index: start_index,
            };
            let end_index = start_index + 1;
            let end_location = Location {
                column: end_index,
                line: 0,
                index: end_index,
            };
            new_tokens.push(LocatedToken {
                token,
                text: "",
                source_span: SourceSpan {
                    start: start_location,
                    end: end_location,
                },
            });
            let span = create_span(&new_tokens);
            //println!("{:#?}", span);
            match parse_file(span) {
                Ok((remaining, parsed)) => {
                    if !remaining.tokens.is_empty() {
                        println!("{:?}", remaining.tokens);
                        println!("{:?}", parsed);
                        panic!("Error, successful parse must consume all tokens");
                    } else {
                        on_found(&new_tokens);
                        queue.push_back((new_tokens, interesting_tokens()));
                    }
                }
                Err(e) => match e.error {
                    parser::core::ParserError::UnexpectedToken(got, expected) => {
                        if got.is_none() {
                            queue.push_back((new_tokens, expected));
                        }
                    }
                },
            }
        }
    }
}

fn get_sample_token(kind: TokenKind) -> Token<'static> {
    match kind {
        TokenKind::Ident => Token::Ident("example_identifier"),
        TokenKind::Number => Token::Number("42"),
        TokenKind::String => Token::String("example string"),
        TokenKind::RawString => Token::RawString("r#example raw string#"),
        TokenKind::Label => Token::Label("'example_label"),
        TokenKind::Boolean => Token::Boolean(true),
        TokenKind::Range => Token::Range,
        TokenKind::And => Token::And,
        TokenKind::Or => Token::Or,
        TokenKind::Equal => Token::Equal,
        TokenKind::NotEqual => Token::NotEqual,
        TokenKind::LessThan => Token::LessThan,
        TokenKind::LessThanOrEqual => Token::LessThanOrEqual,
        TokenKind::GreaterThan => Token::GreaterThan,
        TokenKind::GreaterThanOrEqual => Token::GreaterThanOrEqual,
        TokenKind::Coalesce => Token::Coalesce,
        TokenKind::Divide => Token::Divide,
        TokenKind::Modulo => Token::Modulo,
        TokenKind::Power => Token::Power,
        TokenKind::Pipe => Token::Pipe,
        TokenKind::Caret => Token::Caret,
        TokenKind::Plus => Token::Plus,
        TokenKind::Minus => Token::Minus,
        TokenKind::Star => Token::Star,
        TokenKind::PlusPercent => Token::PlusPercent,
        TokenKind::MinusPercent => Token::MinusPercent,
        TokenKind::StarPercent => Token::StarPercent,
        TokenKind::Exclamation => Token::Exclamation,
        TokenKind::Question => Token::Question,
        TokenKind::Dot => Token::Dot,
        TokenKind::Comma => Token::Comma,
        TokenKind::Colon => Token::Colon,
        TokenKind::Semicolon => Token::Semicolon,
        TokenKind::LeftParen => Token::LeftParen,
        TokenKind::RightParen => Token::RightParen,
        TokenKind::LeftBracket => Token::LeftBracket,
        TokenKind::RightBracket => Token::RightBracket,
        TokenKind::LeftBrace => Token::LeftBrace,
        TokenKind::RightBrace => Token::RightBrace,
        TokenKind::Assignment => Token::Assignment,
        TokenKind::Arrow => Token::Arrow,
        TokenKind::VerticalBar => Token::VerticalBar,
        TokenKind::Ampersand => Token::Ampersand,
        TokenKind::Let => Token::Let,
        TokenKind::If => Token::If,
        TokenKind::Else => Token::Else,
        TokenKind::While => Token::While,
        TokenKind::For => Token::For,
        TokenKind::Loop => Token::Loop,
        TokenKind::In => Token::In,
        TokenKind::Break => Token::Break,
        TokenKind::Continue => Token::Continue,
        TokenKind::Return => Token::Return,
        TokenKind::Struct => Token::Struct,
        TokenKind::Enum => Token::Enum,
        TokenKind::Impl => Token::Impl,
        TokenKind::Trait => Token::Trait,
        TokenKind::Mod => Token::Mod,
        TokenKind::Type => Token::Type,
        TokenKind::Fn => Token::Fn,
        TokenKind::Mut => Token::Mut,
        TokenKind::Import => Token::Import,
        TokenKind::As => Token::As,
        TokenKind::Do => Token::Do,
        TokenKind::UseEnv => Token::UseEnv,
        TokenKind::LineComment => Token::LineComment("// Example line comment"),
        TokenKind::BlockComment => Token::BlockComment("/* Example block comment */"),
        TokenKind::Space => Token::Space(" "),
        TokenKind::Error => Token::Error,
    }
}

fn iter_tokens(set: EnumSet<TokenKind>) -> impl Iterator<Item = Token<'static>> {
    set.into_iter().map(|kind| get_sample_token(kind))
}

pub fn interesting_tokens() -> EnumSet<TokenKind> {
    let mut set = EnumSet::empty();
    set.insert(TokenKind::Ident);
    set.insert(TokenKind::Number);
    set.insert(TokenKind::Label);
    set.insert(TokenKind::Equal);
    set.insert(TokenKind::Coalesce);
    set.insert(TokenKind::Power);
    set.insert(TokenKind::Pipe);
    set.insert(TokenKind::Plus);
    set.insert(TokenKind::Question);
    set.insert(TokenKind::Comma);
    set.insert(TokenKind::Colon);
    set.insert(TokenKind::Semicolon);
    set.insert(TokenKind::LeftParen);
    set.insert(TokenKind::RightParen);
    set.insert(TokenKind::LeftBracket);
    set.insert(TokenKind::RightBracket);
    set.insert(TokenKind::LeftBrace);
    set.insert(TokenKind::RightBrace);
    set.insert(TokenKind::Arrow);
    set.insert(TokenKind::Assignment);
    set.insert(TokenKind::Let);
    set.insert(TokenKind::If);
    set.insert(TokenKind::Else);
    set.insert(TokenKind::While);
    set.insert(TokenKind::For);
    set.insert(TokenKind::Loop);
    set.insert(TokenKind::In);
    set.insert(TokenKind::Break);
    set.insert(TokenKind::Continue);
    set.insert(TokenKind::Return);
    set.insert(TokenKind::Struct);
    set.insert(TokenKind::Enum);
    set.insert(TokenKind::Impl);
    set.insert(TokenKind::Trait);
    set.insert(TokenKind::Mod);
    set.insert(TokenKind::Type);
    set.insert(TokenKind::Fn);
    set.insert(TokenKind::Mut);
    set.insert(TokenKind::Import);
    set.insert(TokenKind::As);
    set.insert(TokenKind::Do);
    set.insert(TokenKind::UseEnv);
    set.insert(TokenKind::Space);
    set.insert(TokenKind::Error);
    set
}
