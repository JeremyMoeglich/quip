mod lexer {
    use crate::*;
    use ast::Number;
    use enum_kinds::EnumKind;
    use logos::{internal::LexerInternal, Lexer, Logos};
    use num::{BigInt, Num};
    use proc_macros::TokenParser;
    use std::str::FromStr;
    #[enum_kind(TokenKind)]
    pub enum Token<'a> {
        #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
        Ident(&'a str),
        #[regex(
            r"0[xX][0-9a-fA-F]+|0[bB][01]+|0[oO][0-7]+|(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?",
            number
        )]
        Number(Number),
        #[regex(r#""([^"\\]|\\.)*""#)]
        DoubleQuoteString(&'a str),
        #[regex(r#"'([^'\\]|\\.)*'"#)]
        SingleQuoteString(&'a str),
        #[regex("r#", raw_string_start)]
        RawString(&'a str),
        #[token("true|false", |lex|{if lex.slice()= = "true"{true}else{false}})]
        Boolean(bool),
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
        Arrow,
        #[token("|")]
        VerticalBar,
        #[token("&")]
        Ampersand,
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
        #[token("struct")]
        Struct,
        #[token("enum")]
        Enum,
        #[token("impl")]
        Impl,
        #[token("type")]
        Type,
        #[token("fn")]
        Fn,
        #[token("mut")]
        Mut,
        #[regex(r"//.*")]
        LineComment(&'a str),
        #[regex(r"/\*", block_comment)]
        BlockComment(&'a str),
        #[regex(r"( |\n|\t)*")]
        Space(&'a str),
        EOF,
        Error,
    }
    impl<'s> ::logos::Logos<'s> for Token<'s> {
        type Error = ();
        type Extras = ();
        type Source = str;
        fn lex(lex: &mut ::logos::Lexer<'s, Self>) {
            use logos::internal::{CallbackResult, LexerInternal};
            type Lexer<'s> = ::logos::Lexer<'s, Token<'s>>;
            fn _end<'s>(lex: &mut Lexer<'s>) {
                lex.end()
            }
            fn _error<'s>(lex: &mut Lexer<'s>) {
                lex.bump_unchecked(1);
                lex.error();
            }
            static COMPACT_TABLE_0: [u8; 256] = [
                17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
                17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 16, 17, 17, 17, 17, 1, 17, 17,
                17, 17, 17, 17, 17, 17, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 17, 17,
                17, 17, 17, 17, 17, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
                127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 17, 0, 17,
                17, 127, 17, 127, 127, 127, 127, 123, 127, 127, 127, 127, 127, 127, 127, 127, 127,
                127, 127, 127, 63, 95, 127, 119, 127, 127, 127, 127, 127, 17, 17, 17, 17, 17, 17,
                17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
                17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
                17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
                17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
                17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
                17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
                17,
            ];
            #[inline]
            fn goto721_x<'s>(lex: &mut Lexer<'s>) {
                let token = Token::DoubleQuoteString(lex.slice());
                lex.set(Ok(token));
            }
            #[inline]
            fn goto722_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"\"") => {
                        lex.bump_unchecked(1usize);
                        goto721_x(lex)
                    }
                    _ => lex.error(),
                }
            }
            #[inline]
            fn goto721_ctx722_x<'s>(lex: &mut Lexer<'s>) {
                let token = Token::DoubleQuoteString(lex.slice());
                lex.set(Ok(token));
            }
            #[inline]
            fn goto722_ctx722_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"\"") => {
                        lex.bump_unchecked(1usize);
                        goto721_ctx722_x(lex)
                    }
                    _ => goto722_x(lex),
                }
            }
            #[inline]
            fn pattern0(byte: u8) -> bool {
                COMPACT_TABLE_0[byte as usize] & 1 > 0
            }
            #[inline]
            fn pattern1(byte: u8) -> bool {
                match byte {
                    0u8..=9u8 | 11u8..=255u8 => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto725_at1_ctx722_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto722_x(lex),
                };
                match byte {
                    byte if pattern1(byte) => {
                        lex.bump_unchecked(2usize);
                        goto723_ctx722_x(lex)
                    }
                    _ => goto722_x(lex),
                }
            }
            #[inline]
            fn goto723_ctx722_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto722_ctx722_x(lex),
                };
                match byte {
                    byte if pattern0(byte) => {
                        lex.bump_unchecked(1usize);
                        goto723_ctx722_x(lex)
                    }
                    b'\\' => goto725_at1_ctx722_x(lex),
                    _ => goto722_ctx722_x(lex),
                }
            }
            #[inline]
            fn goto1_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let token = Token::Ident(lex.slice());
                lex.set(Ok(token));
            }
            #[inline]
            fn goto735_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                raw_string_start(lex).construct(Token::RawString, lex);
            }
            #[inline]
            fn pattern2(byte: u8) -> bool {
                COMPACT_TABLE_0[byte as usize] & 2 > 0
            }
            #[inline]
            fn goto2_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                while let Some(arr) = lex.read::<&[u8; 16]>() {
                    if pattern2(arr[0]) {
                        if pattern2(arr[1]) {
                            if pattern2(arr[2]) {
                                if pattern2(arr[3]) {
                                    if pattern2(arr[4]) {
                                        if pattern2(arr[5]) {
                                            if pattern2(arr[6]) {
                                                if pattern2(arr[7]) {
                                                    if pattern2(arr[8]) {
                                                        if pattern2(arr[9]) {
                                                            if pattern2(arr[10]) {
                                                                if pattern2(arr[11]) {
                                                                    if pattern2(arr[12]) {
                                                                        if pattern2(arr[13]) {
                                                                            if pattern2(arr[14]) {
                                                                                if pattern2(arr[15])
                                                                                {
                                                                                    lex.bump_unchecked(16);
                                                                                    continue;
                                                                                }
                                                                                lex.bump_unchecked(
                                                                                    15,
                                                                                );
                                                                                return goto1_ctx1_x(lex);
                                                                            }
                                                                            lex.bump_unchecked(14);
                                                                            return goto1_ctx1_x(
                                                                                lex,
                                                                            );
                                                                        }
                                                                        lex.bump_unchecked(13);
                                                                        return goto1_ctx1_x(lex);
                                                                    }
                                                                    lex.bump_unchecked(12);
                                                                    return goto1_ctx1_x(lex);
                                                                }
                                                                lex.bump_unchecked(11);
                                                                return goto1_ctx1_x(lex);
                                                            }
                                                            lex.bump_unchecked(10);
                                                            return goto1_ctx1_x(lex);
                                                        }
                                                        lex.bump_unchecked(9);
                                                        return goto1_ctx1_x(lex);
                                                    }
                                                    lex.bump_unchecked(8);
                                                    return goto1_ctx1_x(lex);
                                                }
                                                lex.bump_unchecked(7);
                                                return goto1_ctx1_x(lex);
                                            }
                                            lex.bump_unchecked(6);
                                            return goto1_ctx1_x(lex);
                                        }
                                        lex.bump_unchecked(5);
                                        return goto1_ctx1_x(lex);
                                    }
                                    lex.bump_unchecked(4);
                                    return goto1_ctx1_x(lex);
                                }
                                lex.bump_unchecked(3);
                                return goto1_ctx1_x(lex);
                            }
                            lex.bump_unchecked(2);
                            return goto1_ctx1_x(lex);
                        }
                        lex.bump_unchecked(1);
                        return goto1_ctx1_x(lex);
                    }
                    return goto1_ctx1_x(lex);
                }
                while lex.test(pattern2) {
                    lex.bump_unchecked(1);
                }
                goto1_ctx1_x(lex);
            }
            #[inline]
            fn goto778_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Return));
            }
            #[inline]
            fn goto855_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto778_ctx1_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto778_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto854_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 4usize]>() {
                    Some(b"turn") => {
                        lex.bump_unchecked(4usize);
                        goto855_ctx1_x(lex)
                    }
                    _ => goto2_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto852_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J735,
                    J854,
                    J2,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J735, __,
                        __, __, __, __, __, __, __, __, __, __, __, J2, J2, J2, J2, J2, J2, J2, J2,
                        J2, J2, __, __, __, __, __, __, __, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2,
                        J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, __, __, __,
                        __, J2, __, J2, J2, J2, J2, J854, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2,
                        J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto1_ctx1_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J735 => {
                        lex.bump_unchecked(1usize);
                        goto735_ctx1_x(lex)
                    }
                    Jump::J854 => {
                        lex.bump_unchecked(1usize);
                        goto854_ctx1_x(lex)
                    }
                    Jump::J2 => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    Jump::__ => goto1_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto782_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Type));
            }
            #[inline]
            fn goto882_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto782_ctx1_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto782_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto881_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 2usize]>() {
                    Some(b"pe") => {
                        lex.bump_unchecked(2usize);
                        goto882_ctx1_x(lex)
                    }
                    _ => goto2_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto1_x<'s>(lex: &mut Lexer<'s>) {
                let token = Token::Ident(lex.slice());
                lex.set(Ok(token));
            }
            #[inline]
            fn goto737_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(lex: &mut Lexer<'s>) -> impl CallbackResult<'s, bool, Token<'s>> {
                    if lex.slice() == "true" {
                        true
                    } else {
                        false
                    }
                }
                callback(lex).construct(Token::Boolean, lex);
            }
            #[inline]
            fn goto804_at1_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 5usize]>(1usize) {
                    Some(b"false") => {
                        lex.bump_unchecked(6usize);
                        goto737_ctx1_x(lex)
                    }
                    _ => goto1_x(lex),
                }
            }
            #[inline]
            fn goto803_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto1_ctx1_x(lex),
                };
                match byte {
                    b'|' => goto804_at1_ctx1_x(lex),
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto1_ctx1_x(lex),
                }
            }
            #[inline]
            fn pattern3(byte: u8) -> bool {
                COMPACT_TABLE_0[byte as usize] & 4 > 0
            }
            #[inline]
            fn goto880_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto1_ctx1_x(lex),
                };
                match byte {
                    b'e' => {
                        lex.bump_unchecked(1usize);
                        goto803_ctx1_x(lex)
                    }
                    byte if pattern3(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto1_ctx1_x(lex),
                }
            }
            #[inline]
            fn pattern4(byte: u8) -> bool {
                COMPACT_TABLE_0[byte as usize] & 8 > 0
            }
            #[inline]
            fn goto878_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto1_ctx1_x(lex),
                };
                match byte {
                    b'u' => {
                        lex.bump_unchecked(1usize);
                        goto880_ctx1_x(lex)
                    }
                    byte if pattern4(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto1_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto875_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J881,
                    J878,
                    J2,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, J2, J2, J2, J2, J2, J2, J2, J2, J2,
                        J2, __, __, __, __, __, __, __, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2,
                        J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, __, __, __, __,
                        J2, __, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2,
                        J878, J2, J2, J2, J2, J2, J2, J881, J2, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto1_ctx1_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J881 => {
                        lex.bump_unchecked(1usize);
                        goto881_ctx1_x(lex)
                    }
                    Jump::J878 => {
                        lex.bump_unchecked(1usize);
                        goto878_ctx1_x(lex)
                    }
                    Jump::J2 => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    Jump::__ => goto1_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto728_x<'s>(lex: &mut Lexer<'s>) {
                let token = Token::SingleQuoteString(lex.slice());
                lex.set(Ok(token));
            }
            #[inline]
            fn goto729_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"'") => {
                        lex.bump_unchecked(1usize);
                        goto728_x(lex)
                    }
                    _ => lex.error(),
                }
            }
            #[inline]
            fn goto728_ctx729_x<'s>(lex: &mut Lexer<'s>) {
                let token = Token::SingleQuoteString(lex.slice());
                lex.set(Ok(token));
            }
            #[inline]
            fn goto729_ctx729_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"'") => {
                        lex.bump_unchecked(1usize);
                        goto728_ctx729_x(lex)
                    }
                    _ => goto729_x(lex),
                }
            }
            #[inline]
            fn pattern5(byte: u8) -> bool {
                COMPACT_TABLE_0[byte as usize] & 16 > 0
            }
            #[inline]
            fn goto732_at1_ctx729_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto729_x(lex),
                };
                match byte {
                    byte if pattern1(byte) => {
                        lex.bump_unchecked(2usize);
                        goto730_ctx729_x(lex)
                    }
                    _ => goto729_x(lex),
                }
            }
            #[inline]
            fn goto730_ctx729_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto729_ctx729_x(lex),
                };
                match byte {
                    byte if pattern5(byte) => {
                        lex.bump_unchecked(1usize);
                        goto730_ctx729_x(lex)
                    }
                    b'\\' => goto732_at1_ctx729_x(lex),
                    _ => goto729_ctx729_x(lex),
                }
            }
            #[inline]
            fn goto776_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Break));
            }
            #[inline]
            fn goto847_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto776_ctx2_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto776_ctx2_x(lex),
                }
            }
            #[inline]
            fn goto846_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 4usize]>() {
                    Some(b"reak") => {
                        lex.bump_unchecked(4usize);
                        goto847_ctx2_x(lex)
                    }
                    _ => goto2_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto772_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Else));
            }
            #[inline]
            fn goto835_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto772_ctx1_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto772_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto866_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto1_ctx1_x(lex),
                };
                match byte {
                    b'e' => {
                        lex.bump_unchecked(1usize);
                        goto835_ctx1_x(lex)
                    }
                    byte if pattern3(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto1_ctx1_x(lex),
                }
            }
            #[inline]
            fn pattern6(byte: u8) -> bool {
                COMPACT_TABLE_0[byte as usize] & 32 > 0
            }
            #[inline]
            fn goto863_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto1_ctx1_x(lex),
                };
                match byte {
                    b's' => {
                        lex.bump_unchecked(1usize);
                        goto866_ctx1_x(lex)
                    }
                    byte if pattern6(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto1_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto780_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Enum));
            }
            #[inline]
            fn goto868_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto780_ctx1_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto780_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto867_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 2usize]>() {
                    Some(b"um") => {
                        lex.bump_unchecked(2usize);
                        goto868_ctx1_x(lex)
                    }
                    _ => goto2_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto860_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J863,
                    J867,
                    J2,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, J2, J2, J2, J2, J2, J2, J2, J2, J2,
                        J2, __, __, __, __, __, __, __, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2,
                        J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, __, __, __, __,
                        J2, __, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J863, J2, J867, J2, J2,
                        J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto1_ctx1_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J863 => {
                        lex.bump_unchecked(1usize);
                        goto863_ctx1_x(lex)
                    }
                    Jump::J867 => {
                        lex.bump_unchecked(1usize);
                        goto867_ctx1_x(lex)
                    }
                    Jump::J2 => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    Jump::__ => goto1_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                number(lex).construct(Token::Number, lex);
            }
            #[inline]
            fn goto4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                number(lex).construct(Token::Number, lex);
            }
            #[inline]
            fn goto4_x<'s>(lex: &mut Lexer<'s>) {
                number(lex).construct(Token::Number, lex);
            }
            #[inline]
            fn goto21_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(2usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto20_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(2usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto45_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto20_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto22_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn pattern7(byte: u8) -> bool {
                match byte {
                    144u8..=153u8 | 176u8..=185u8 => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto83_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    byte if pattern7(byte) => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto87_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J21,
                    J22,
                    J83,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J20, __, __, __, __, __, __, __, __, __, __, J45, J22, __, __, J83, __,
                        J45, __, __, __, __, __, J21, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at2_ctx4_x(lex),
                    Jump::J20 => goto20_at2_ctx4_x(lex),
                    Jump::J21 => goto21_at2_ctx4_x(lex),
                    Jump::J22 => goto22_at2_ctx4_x(lex),
                    Jump::J83 => goto83_at2_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto22_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(2usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto26_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto49_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J26,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J26, __, J26, __, J26,
                        __, J26, __, J26, __, J26, __, J26, __, J26, __, J26, __, J26, __, J45, __,
                        J45, J20, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at2_ctx4_x(lex),
                    Jump::J20 => goto20_at2_ctx4_x(lex),
                    Jump::J26 => goto26_at2_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto21_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto26_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto45_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto22_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto20_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto106_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto157_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J26,
                    J45,
                    J22,
                    J20,
                    J106,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J26, __, J21,
                        J106, __, __, J45, __, __, __, J21, __, __, __, __, __, J45, __, J45, __,
                        __, __, __, __, J45, __, J22, J21, __, __, __, __, __, __, J20, __, J45,
                        __, __, __, __, __, __, __, __, __, __, __, J45, __, __, __, J45, J20, __,
                        __, __, __, __, __, J45, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at3_ctx4_x(lex),
                    Jump::J26 => goto26_at3_ctx4_x(lex),
                    Jump::J45 => goto45_at3_ctx4_x(lex),
                    Jump::J22 => goto22_at3_ctx4_x(lex),
                    Jump::J20 => goto20_at3_ctx4_x(lex),
                    Jump::J106 => goto106_at3_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto167_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J20,
                        __, J22, __, J45, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at3_ctx4_x(lex),
                    Jump::J20 => goto20_at3_ctx4_x(lex),
                    Jump::J22 => goto22_at3_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto94_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    180u8 => goto21_at3_ctx4_x(lex),
                    146u8 => goto20_at3_ctx4_x(lex),
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto183_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J45,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J22, __, __, __, __, __, J21, __, __, __, __, __, __, __, J21, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, J45, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at3_ctx4_x(lex),
                    Jump::J45 => goto45_at3_ctx4_x(lex),
                    Jump::J22 => goto22_at3_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto86_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto170_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto185_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J157,
                    J167,
                    J94,
                    J183,
                    J86,
                    J170,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J94, J157, __, __, __, __,
                        J167, __, __, __, __, __, __, J170, J183, J86, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J157 => goto157_at2_ctx4_x(lex),
                    Jump::J167 => goto167_at2_ctx4_x(lex),
                    Jump::J94 => goto94_at2_ctx4_x(lex),
                    Jump::J183 => goto183_at2_ctx4_x(lex),
                    Jump::J86 => goto86_at2_ctx4_x(lex),
                    Jump::J170 => goto170_at2_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto88_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(1usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto59_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn pattern8(byte: u8) -> bool {
                match byte {
                    128u8..=137u8 | 144u8..=153u8 => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto66_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    byte if pattern8(byte) => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto74_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J21,
                    J22,
                    J59,
                    J66,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J22, J45, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J20, J45, __, __, __, __, J59, __, J45, __,
                        __, J66, __, __, J45, J21, __, __, J66, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at2_ctx4_x(lex),
                    Jump::J20 => goto20_at2_ctx4_x(lex),
                    Jump::J21 => goto21_at2_ctx4_x(lex),
                    Jump::J22 => goto22_at2_ctx4_x(lex),
                    Jump::J59 => goto59_at2_ctx4_x(lex),
                    Jump::J66 => goto66_at2_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto19_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J20,
                    J87,
                    J22,
                    J49,
                    J19,
                    J185,
                    J88,
                    J74,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, J19, J19, J19, J19, J19, J19, J19,
                        J19, J19, J19, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J20, __, J21, __, __, __, J22,
                        J49, J74, __, __, __, __, __, __, __, __, J87, __, __, __, __, J88, J185,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto4_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at1_ctx4_x(lex),
                    Jump::J20 => goto20_at1_ctx4_x(lex),
                    Jump::J87 => goto87_at1_ctx4_x(lex),
                    Jump::J22 => goto22_at1_ctx4_x(lex),
                    Jump::J49 => goto49_at1_ctx4_x(lex),
                    Jump::J19 => {
                        lex.bump_unchecked(1usize);
                        goto19_ctx4_x(lex)
                    }
                    Jump::J185 => goto185_at1_ctx4_x(lex),
                    Jump::J88 => goto88_at1_ctx4_x(lex),
                    Jump::J74 => goto74_at1_ctx4_x(lex),
                    Jump::__ => goto4_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto21_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto83_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    byte if pattern7(byte) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto87_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J21,
                    J22,
                    J83,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J20, __, __, __, __, __, __, __, __, __, __, J45, J22, __, __, J83, __,
                        J45, __, __, __, __, __, J21, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at3_ctx4_x(lex),
                    Jump::J20 => goto20_at3_ctx4_x(lex),
                    Jump::J21 => goto21_at3_ctx4_x(lex),
                    Jump::J22 => goto22_at3_ctx4_x(lex),
                    Jump::J83 => goto83_at3_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto49_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J26,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J26, __, J26, __, J26,
                        __, J26, __, J26, __, J26, __, J26, __, J26, __, J26, __, J26, __, J45, __,
                        J45, J20, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at3_ctx4_x(lex),
                    Jump::J20 => goto20_at3_ctx4_x(lex),
                    Jump::J26 => goto26_at3_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto21_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto26_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto45_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto22_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto20_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto106_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto157_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J26,
                    J45,
                    J22,
                    J20,
                    J106,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J26, __, J21,
                        J106, __, __, J45, __, __, __, J21, __, __, __, __, __, J45, __, J45, __,
                        __, __, __, __, J45, __, J22, J21, __, __, __, __, __, __, J20, __, J45,
                        __, __, __, __, __, __, __, __, __, __, __, J45, __, __, __, J45, J20, __,
                        __, __, __, __, __, J45, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at4_ctx4_x(lex),
                    Jump::J26 => goto26_at4_ctx4_x(lex),
                    Jump::J45 => goto45_at4_ctx4_x(lex),
                    Jump::J22 => goto22_at4_ctx4_x(lex),
                    Jump::J20 => goto20_at4_ctx4_x(lex),
                    Jump::J106 => goto106_at4_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto167_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J20,
                        __, J22, __, J45, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at4_ctx4_x(lex),
                    Jump::J20 => goto20_at4_ctx4_x(lex),
                    Jump::J22 => goto22_at4_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto94_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    180u8 => goto21_at4_ctx4_x(lex),
                    146u8 => goto20_at4_ctx4_x(lex),
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto183_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J45,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J22, __, __, __, __, __, J21, __, __, __, __, __, __, __, J21, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, J45, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at4_ctx4_x(lex),
                    Jump::J45 => goto45_at4_ctx4_x(lex),
                    Jump::J22 => goto22_at4_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto86_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(3usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto170_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(3usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto185_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J157,
                    J167,
                    J94,
                    J183,
                    J86,
                    J170,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J94, J157, __, __, __, __,
                        J167, __, __, __, __, __, __, J170, J183, J86, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J157 => goto157_at3_ctx4_x(lex),
                    Jump::J167 => goto167_at3_ctx4_x(lex),
                    Jump::J94 => goto94_at3_ctx4_x(lex),
                    Jump::J183 => goto183_at3_ctx4_x(lex),
                    Jump::J86 => goto86_at3_ctx4_x(lex),
                    Jump::J170 => goto170_at3_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto88_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto59_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto66_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    byte if pattern8(byte) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto74_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J21,
                    J22,
                    J59,
                    J66,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J22, J45, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J20, J45, __, __, __, __, J59, __, J45, __,
                        __, J66, __, __, J45, J21, __, __, J66, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at3_ctx4_x(lex),
                    Jump::J20 => goto20_at3_ctx4_x(lex),
                    Jump::J21 => goto21_at3_ctx4_x(lex),
                    Jump::J22 => goto22_at3_ctx4_x(lex),
                    Jump::J59 => goto59_at3_ctx4_x(lex),
                    Jump::J66 => goto66_at3_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto186_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J20,
                    J87,
                    J22,
                    J49,
                    J19,
                    J185,
                    J88,
                    J74,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, J19, J19, J19, J19, J19, J19, J19,
                        J19, J19, J19, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J20, __, J21, __, __, __, J22,
                        J49, J74, __, __, __, __, __, __, __, __, J87, __, __, __, __, J88, J185,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at2_ctx4_x(lex),
                    Jump::J20 => goto20_at2_ctx4_x(lex),
                    Jump::J87 => goto87_at2_ctx4_x(lex),
                    Jump::J22 => goto22_at2_ctx4_x(lex),
                    Jump::J49 => goto49_at2_ctx4_x(lex),
                    Jump::J19 => {
                        lex.bump_unchecked(2usize);
                        goto19_ctx4_x(lex)
                    }
                    Jump::J185 => goto185_at2_ctx4_x(lex),
                    Jump::J88 => goto88_at2_ctx4_x(lex),
                    Jump::J74 => goto74_at2_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto83_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    byte if pattern7(byte) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto87_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J21,
                    J22,
                    J83,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J20, __, __, __, __, __, __, __, __, __, __, J45, J22, __, __, J83, __,
                        J45, __, __, __, __, __, J21, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at4_ctx4_x(lex),
                    Jump::J20 => goto20_at4_ctx4_x(lex),
                    Jump::J21 => goto21_at4_ctx4_x(lex),
                    Jump::J22 => goto22_at4_ctx4_x(lex),
                    Jump::J83 => goto83_at4_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto49_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J26,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J26, __, J26, __, J26,
                        __, J26, __, J26, __, J26, __, J26, __, J26, __, J26, __, J26, __, J45, __,
                        J45, J20, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at4_ctx4_x(lex),
                    Jump::J20 => goto20_at4_ctx4_x(lex),
                    Jump::J26 => goto26_at4_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto21_at5_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(5usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto26_at5_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(5usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto45_at5_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(5usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto22_at5_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(5usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto20_at5_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(5usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto106_at5_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(5usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto157_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J26,
                    J45,
                    J22,
                    J20,
                    J106,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J26, __, J21,
                        J106, __, __, J45, __, __, __, J21, __, __, __, __, __, J45, __, J45, __,
                        __, __, __, __, J45, __, J22, J21, __, __, __, __, __, __, J20, __, J45,
                        __, __, __, __, __, __, __, __, __, __, __, J45, __, __, __, J45, J20, __,
                        __, __, __, __, __, J45, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at5_ctx4_x(lex),
                    Jump::J26 => goto26_at5_ctx4_x(lex),
                    Jump::J45 => goto45_at5_ctx4_x(lex),
                    Jump::J22 => goto22_at5_ctx4_x(lex),
                    Jump::J20 => goto20_at5_ctx4_x(lex),
                    Jump::J106 => goto106_at5_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto167_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J20,
                        __, J22, __, J45, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at5_ctx4_x(lex),
                    Jump::J20 => goto20_at5_ctx4_x(lex),
                    Jump::J22 => goto22_at5_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto94_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    180u8 => goto21_at5_ctx4_x(lex),
                    146u8 => goto20_at5_ctx4_x(lex),
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto183_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J45,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J22, __, __, __, __, __, J21, __, __, __, __, __, __, __, J21, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, J45, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at5_ctx4_x(lex),
                    Jump::J45 => goto45_at5_ctx4_x(lex),
                    Jump::J22 => goto22_at5_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto86_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(4usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto170_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(4usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto185_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J157,
                    J167,
                    J94,
                    J183,
                    J86,
                    J170,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J94, J157, __, __, __, __,
                        J167, __, __, __, __, __, __, J170, J183, J86, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J157 => goto157_at4_ctx4_x(lex),
                    Jump::J167 => goto167_at4_ctx4_x(lex),
                    Jump::J94 => goto94_at4_ctx4_x(lex),
                    Jump::J183 => goto183_at4_ctx4_x(lex),
                    Jump::J86 => goto86_at4_ctx4_x(lex),
                    Jump::J170 => goto170_at4_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto88_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(3usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto59_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto66_at4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    byte if pattern8(byte) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto74_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J21,
                    J22,
                    J59,
                    J66,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J22, J45, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J20, J45, __, __, __, __, J59, __, J45, __,
                        __, J66, __, __, J45, J21, __, __, J66, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at4_ctx4_x(lex),
                    Jump::J20 => goto20_at4_ctx4_x(lex),
                    Jump::J21 => goto21_at4_ctx4_x(lex),
                    Jump::J22 => goto22_at4_ctx4_x(lex),
                    Jump::J59 => goto59_at4_ctx4_x(lex),
                    Jump::J66 => goto66_at4_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto186_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J20,
                    J87,
                    J22,
                    J49,
                    J19,
                    J185,
                    J88,
                    J74,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, J19, J19, J19, J19, J19, J19, J19,
                        J19, J19, J19, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J20, __, J21, __, __, __, J22,
                        J49, J74, __, __, __, __, __, __, __, __, J87, __, __, __, __, J88, J185,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at3_ctx4_x(lex),
                    Jump::J20 => goto20_at3_ctx4_x(lex),
                    Jump::J87 => goto87_at3_ctx4_x(lex),
                    Jump::J22 => goto22_at3_ctx4_x(lex),
                    Jump::J49 => goto49_at3_ctx4_x(lex),
                    Jump::J19 => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    Jump::J185 => goto185_at3_ctx4_x(lex),
                    Jump::J88 => goto88_at3_ctx4_x(lex),
                    Jump::J74 => goto74_at3_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto187_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto186_at1_ctx4_x(lex),
                };
                match byte {
                    b'+' | b'-' => goto186_at2_ctx4_x(lex),
                    _ => goto186_at1_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto188_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto4_ctx4_x(lex),
                };
                match byte {
                    b'E' | b'e' => goto187_at1_ctx4_x(lex),
                    _ => goto4_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto21_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto20_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto45_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto20_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto21_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto22_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto83_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match byte {
                    byte if pattern7(byte) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto87_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J21,
                    J22,
                    J83,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J20, __, __, __, __, __, __, __, __, __, __, J45, J22, __, __, J83, __,
                        J45, __, __, __, __, __, J21, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at3_ctx188_x(lex),
                    Jump::J20 => goto20_at3_ctx188_x(lex),
                    Jump::J21 => goto21_at3_ctx188_x(lex),
                    Jump::J22 => goto22_at3_ctx188_x(lex),
                    Jump::J83 => goto83_at3_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto22_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto26_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto49_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J26,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J26, __, J26, __, J26,
                        __, J26, __, J26, __, J26, __, J26, __, J26, __, J26, __, J26, __, J45, __,
                        J45, J20, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at3_ctx188_x(lex),
                    Jump::J20 => goto20_at3_ctx188_x(lex),
                    Jump::J26 => goto26_at3_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto21_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto26_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto45_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto22_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto20_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto106_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto157_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J26,
                    J45,
                    J22,
                    J20,
                    J106,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J26, __, J21,
                        J106, __, __, J45, __, __, __, J21, __, __, __, __, __, J45, __, J45, __,
                        __, __, __, __, J45, __, J22, J21, __, __, __, __, __, __, J20, __, J45,
                        __, __, __, __, __, __, __, __, __, __, __, J45, __, __, __, J45, J20, __,
                        __, __, __, __, __, J45, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at4_ctx188_x(lex),
                    Jump::J26 => goto26_at4_ctx188_x(lex),
                    Jump::J45 => goto45_at4_ctx188_x(lex),
                    Jump::J22 => goto22_at4_ctx188_x(lex),
                    Jump::J20 => goto20_at4_ctx188_x(lex),
                    Jump::J106 => goto106_at4_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto167_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J20,
                        __, J22, __, J45, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at4_ctx188_x(lex),
                    Jump::J20 => goto20_at4_ctx188_x(lex),
                    Jump::J22 => goto22_at4_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto94_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match byte {
                    180u8 => goto21_at4_ctx188_x(lex),
                    146u8 => goto20_at4_ctx188_x(lex),
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto183_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J45,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J22, __, __, __, __, __, J21, __, __, __, __, __, __, __, J21, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, J45, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at4_ctx188_x(lex),
                    Jump::J45 => goto45_at4_ctx188_x(lex),
                    Jump::J22 => goto22_at4_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto86_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(3usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto170_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(3usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto185_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J157,
                    J167,
                    J94,
                    J183,
                    J86,
                    J170,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J94, J157, __, __, __, __,
                        J167, __, __, __, __, __, __, J170, J183, J86, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J157 => goto157_at3_ctx188_x(lex),
                    Jump::J167 => goto167_at3_ctx188_x(lex),
                    Jump::J94 => goto94_at3_ctx188_x(lex),
                    Jump::J183 => goto183_at3_ctx188_x(lex),
                    Jump::J86 => goto86_at3_ctx188_x(lex),
                    Jump::J170 => goto170_at3_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto88_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto59_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto66_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match byte {
                    byte if pattern8(byte) => {
                        lex.bump_unchecked(4usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto74_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J21,
                    J22,
                    J59,
                    J66,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J22, J45, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J20, J45, __, __, __, __, J59, __, J45, __,
                        __, J66, __, __, J45, J21, __, __, J66, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at3_ctx188_x(lex),
                    Jump::J20 => goto20_at3_ctx188_x(lex),
                    Jump::J21 => goto21_at3_ctx188_x(lex),
                    Jump::J22 => goto22_at3_ctx188_x(lex),
                    Jump::J59 => goto59_at3_ctx188_x(lex),
                    Jump::J66 => goto66_at3_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto186_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J20,
                    J87,
                    J22,
                    J49,
                    J19,
                    J185,
                    J88,
                    J74,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, J19, J19, J19, J19, J19, J19, J19,
                        J19, J19, J19, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J20, __, J21, __, __, __, J22,
                        J49, J74, __, __, __, __, __, __, __, __, J87, __, __, __, __, J88, J185,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at2_ctx188_x(lex),
                    Jump::J20 => goto20_at2_ctx188_x(lex),
                    Jump::J87 => goto87_at2_ctx188_x(lex),
                    Jump::J22 => goto22_at2_ctx188_x(lex),
                    Jump::J49 => goto49_at2_ctx188_x(lex),
                    Jump::J19 => {
                        lex.bump_unchecked(2usize);
                        goto19_ctx4_x(lex)
                    }
                    Jump::J185 => goto185_at2_ctx188_x(lex),
                    Jump::J88 => goto88_at2_ctx188_x(lex),
                    Jump::J74 => goto74_at2_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto83_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match byte {
                    byte if pattern7(byte) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto87_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J21,
                    J22,
                    J83,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J20, __, __, __, __, __, __, __, __, __, __, J45, J22, __, __, J83, __,
                        J45, __, __, __, __, __, J21, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at4_ctx188_x(lex),
                    Jump::J20 => goto20_at4_ctx188_x(lex),
                    Jump::J21 => goto21_at4_ctx188_x(lex),
                    Jump::J22 => goto22_at4_ctx188_x(lex),
                    Jump::J83 => goto83_at4_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto49_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J26,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J26, __, J26, __, J26,
                        __, J26, __, J26, __, J26, __, J26, __, J26, __, J26, __, J26, __, J45, __,
                        J45, J20, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at4_ctx188_x(lex),
                    Jump::J20 => goto20_at4_ctx188_x(lex),
                    Jump::J26 => goto26_at4_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto21_at5_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(5usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto26_at5_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(5usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto45_at5_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(5usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto22_at5_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(5usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto20_at5_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(5usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto106_at5_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(5usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto157_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J26,
                    J45,
                    J22,
                    J20,
                    J106,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J26, __, J21,
                        J106, __, __, J45, __, __, __, J21, __, __, __, __, __, J45, __, J45, __,
                        __, __, __, __, J45, __, J22, J21, __, __, __, __, __, __, J20, __, J45,
                        __, __, __, __, __, __, __, __, __, __, __, J45, __, __, __, J45, J20, __,
                        __, __, __, __, __, J45, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at5_ctx188_x(lex),
                    Jump::J26 => goto26_at5_ctx188_x(lex),
                    Jump::J45 => goto45_at5_ctx188_x(lex),
                    Jump::J22 => goto22_at5_ctx188_x(lex),
                    Jump::J20 => goto20_at5_ctx188_x(lex),
                    Jump::J106 => goto106_at5_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto167_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J20,
                        __, J22, __, J45, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at5_ctx188_x(lex),
                    Jump::J20 => goto20_at5_ctx188_x(lex),
                    Jump::J22 => goto22_at5_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto94_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match byte {
                    180u8 => goto21_at5_ctx188_x(lex),
                    146u8 => goto20_at5_ctx188_x(lex),
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto183_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J45,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J22, __, __, __, __, __, J21, __, __, __, __, __, __, __, J21, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, J45, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at5_ctx188_x(lex),
                    Jump::J45 => goto45_at5_ctx188_x(lex),
                    Jump::J22 => goto22_at5_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto86_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(4usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto170_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(4usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(6usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto185_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J157,
                    J167,
                    J94,
                    J183,
                    J86,
                    J170,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J94, J157, __, __, __, __,
                        J167, __, __, __, __, __, __, J170, J183, J86, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J157 => goto157_at4_ctx188_x(lex),
                    Jump::J167 => goto167_at4_ctx188_x(lex),
                    Jump::J94 => goto94_at4_ctx188_x(lex),
                    Jump::J183 => goto183_at4_ctx188_x(lex),
                    Jump::J86 => goto86_at4_ctx188_x(lex),
                    Jump::J170 => goto170_at4_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto88_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(3usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto59_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(4usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto66_at4_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match byte {
                    byte if pattern8(byte) => {
                        lex.bump_unchecked(5usize);
                        goto19_ctx4_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto74_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J20,
                    J21,
                    J22,
                    J59,
                    J66,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J22, J45, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J20, J45, __, __, __, __, J59, __, J45, __,
                        __, J66, __, __, J45, J21, __, __, J66, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => goto45_at4_ctx188_x(lex),
                    Jump::J20 => goto20_at4_ctx188_x(lex),
                    Jump::J21 => goto21_at4_ctx188_x(lex),
                    Jump::J22 => goto22_at4_ctx188_x(lex),
                    Jump::J59 => goto59_at4_ctx188_x(lex),
                    Jump::J66 => goto66_at4_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto186_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J20,
                    J87,
                    J22,
                    J49,
                    J19,
                    J185,
                    J88,
                    J74,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, J19, J19, J19, J19, J19, J19, J19,
                        J19, J19, J19, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J20, __, J21, __, __, __, J22,
                        J49, J74, __, __, __, __, __, __, __, __, J87, __, __, __, __, J88, J185,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => goto21_at3_ctx188_x(lex),
                    Jump::J20 => goto20_at3_ctx188_x(lex),
                    Jump::J87 => goto87_at3_ctx188_x(lex),
                    Jump::J22 => goto22_at3_ctx188_x(lex),
                    Jump::J49 => goto49_at3_ctx188_x(lex),
                    Jump::J19 => {
                        lex.bump_unchecked(3usize);
                        goto19_ctx4_x(lex)
                    }
                    Jump::J185 => goto185_at3_ctx188_x(lex),
                    Jump::J88 => goto88_at3_ctx188_x(lex),
                    Jump::J74 => goto74_at3_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto187_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto186_at1_ctx188_x(lex),
                };
                match byte {
                    b'+' | b'-' => goto186_at2_ctx188_x(lex),
                    _ => goto186_at1_ctx188_x(lex),
                }
            }
            #[inline]
            fn goto188_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto4_ctx188_x(lex),
                };
                match byte {
                    b'E' | b'e' => goto187_at1_ctx188_x(lex),
                    _ => goto4_ctx188_x(lex),
                }
            }
            #[inline]
            fn goto229_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(3usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto236_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match byte {
                    byte if pattern8(byte) => {
                        lex.bump_unchecked(3usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto215_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto190_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(3usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto191_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(3usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto192_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(3usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto244_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J229,
                    J236,
                    J215,
                    J190,
                    J191,
                    J192,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J192, J215, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J190, J215, __, __, __, __, J229, __, J215,
                        __, __, J236, __, __, J215, J191, __, __, J236, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J229 => goto229_at2_ctx188_x(lex),
                    Jump::J236 => goto236_at2_ctx188_x(lex),
                    Jump::J215 => goto215_at2_ctx188_x(lex),
                    Jump::J190 => goto190_at2_ctx188_x(lex),
                    Jump::J191 => goto191_at2_ctx188_x(lex),
                    Jump::J192 => goto192_at2_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto192_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(4usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto191_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto215_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(4usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto353_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J192,
                    J191,
                    J215,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J192, __, __, __, __, __, J191, __, __, __, __, __, __, __, J191, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J215, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J192 => goto192_at3_ctx188_x(lex),
                    Jump::J191 => goto191_at3_ctx188_x(lex),
                    Jump::J215 => goto215_at3_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto190_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(4usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto264_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match byte {
                    180u8 => goto191_at3_ctx188_x(lex),
                    146u8 => goto190_at3_ctx188_x(lex),
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto256_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto196_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(4usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto276_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto327_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J196,
                    J191,
                    J215,
                    J190,
                    J192,
                    J276,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J196, __, J191,
                        J276, __, __, J215, __, __, __, J191, __, __, __, __, __, J215, __, J215,
                        __, __, __, __, __, J215, __, J192, J191, __, __, __, __, __, __, J190, __,
                        J215, __, __, __, __, __, __, __, __, __, __, __, J215, __, __, __, J215,
                        J190, __, __, __, __, __, __, J215, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J196 => goto196_at3_ctx188_x(lex),
                    Jump::J191 => goto191_at3_ctx188_x(lex),
                    Jump::J215 => goto215_at3_ctx188_x(lex),
                    Jump::J190 => goto190_at3_ctx188_x(lex),
                    Jump::J192 => goto192_at3_ctx188_x(lex),
                    Jump::J276 => goto276_at3_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto340_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto337_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J192,
                    J215,
                    J190,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J190,
                        __, J192, __, J215, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J192 => goto192_at3_ctx188_x(lex),
                    Jump::J215 => goto215_at3_ctx188_x(lex),
                    Jump::J190 => goto190_at3_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto355_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J353,
                    J264,
                    J256,
                    J327,
                    J340,
                    J337,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J264, J327, __, __, __, __,
                        J337, __, __, __, __, __, __, J340, J353, J256, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J353 => goto353_at2_ctx188_x(lex),
                    Jump::J264 => goto264_at2_ctx188_x(lex),
                    Jump::J256 => goto256_at2_ctx188_x(lex),
                    Jump::J327 => goto327_at2_ctx188_x(lex),
                    Jump::J340 => goto340_at2_ctx188_x(lex),
                    Jump::J337 => goto337_at2_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto192_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(2usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto191_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(2usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto190_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(2usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto258_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(1usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto196_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(3usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto219_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J196,
                    J215,
                    J190,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J196, __, J196, __,
                        J196, __, J196, __, J196, __, J196, __, J196, __, J196, __, J196, __, J196,
                        __, J215, __, J215, J190, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J196 => goto196_at2_ctx188_x(lex),
                    Jump::J215 => goto215_at2_ctx188_x(lex),
                    Jump::J190 => goto190_at2_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto253_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match byte {
                    byte if pattern7(byte) => {
                        lex.bump_unchecked(3usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto257_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J253,
                    J215,
                    J190,
                    J191,
                    J192,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J190, __, __, __, __, __, __, __, __, __, __, J215, J192, __, __, J253, __,
                        J215, __, __, __, __, __, J191, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J253 => goto253_at2_ctx188_x(lex),
                    Jump::J215 => goto215_at2_ctx188_x(lex),
                    Jump::J190 => goto190_at2_ctx188_x(lex),
                    Jump::J191 => goto191_at2_ctx188_x(lex),
                    Jump::J192 => goto192_at2_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto189_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J244,
                    J355,
                    J192,
                    J189,
                    J191,
                    J190,
                    J258,
                    J219,
                    J257,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, J189, J189, J189, J189, J189, J189,
                        J189, J189, J189, J189, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J190, __, J191, __, __,
                        __, J192, J219, J244, __, __, __, __, __, __, __, __, J257, __, __, __, __,
                        J258, J355, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto188_ctx188_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J244 => goto244_at1_ctx188_x(lex),
                    Jump::J355 => goto355_at1_ctx188_x(lex),
                    Jump::J192 => goto192_at1_ctx188_x(lex),
                    Jump::J189 => {
                        lex.bump_unchecked(1usize);
                        goto189_ctx188_x(lex)
                    }
                    Jump::J191 => goto191_at1_ctx188_x(lex),
                    Jump::J190 => goto190_at1_ctx188_x(lex),
                    Jump::J258 => goto258_at1_ctx188_x(lex),
                    Jump::J219 => goto219_at1_ctx188_x(lex),
                    Jump::J257 => goto257_at1_ctx188_x(lex),
                    Jump::__ => goto188_ctx188_x(lex),
                }
            }
            #[inline]
            fn goto356_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b".") => {
                        lex.bump_unchecked(1usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto189_ctx188_x(lex),
                }
            }
            #[inline]
            fn goto356_ctx189_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b".") => {
                        lex.bump_unchecked(1usize);
                        goto189_ctx188_x(lex)
                    }
                    _ => goto189_ctx188_x(lex),
                }
            }
            #[inline]
            fn goto358_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto383_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto364_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto387_at1_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J358,
                    J383,
                    J364,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J364, __, J364, __,
                        J364, __, J364, __, J364, __, J364, __, J364, __, J364, __, J364, __, J364,
                        __, J383, __, J383, J358, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto356_ctx189_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J358 => goto358_at2_ctx356_x(lex),
                    Jump::J383 => goto383_at2_ctx356_x(lex),
                    Jump::J364 => goto364_at2_ctx356_x(lex),
                    Jump::__ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto360_at1_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(2usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto426_at1_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(1usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto360_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto421_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto356_ctx189_x(lex),
                };
                match byte {
                    byte if pattern7(byte) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto359_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto425_at1_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J358,
                    J360,
                    J421,
                    J383,
                    J359,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J358, __, __, __, __, __, __, __, __, __, __, J383, J360, __, __, J421, __,
                        J383, __, __, __, __, __, J359, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto356_ctx189_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J358 => goto358_at2_ctx356_x(lex),
                    Jump::J360 => goto360_at2_ctx356_x(lex),
                    Jump::J421 => goto421_at2_ctx356_x(lex),
                    Jump::J383 => goto383_at2_ctx356_x(lex),
                    Jump::J359 => goto359_at2_ctx356_x(lex),
                    Jump::__ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto358_at1_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(2usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto358_at3_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto359_at3_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto432_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto356_ctx189_x(lex),
                };
                match byte {
                    146u8 => goto358_at3_ctx356_x(lex),
                    180u8 => goto359_at3_ctx356_x(lex),
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto360_at3_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto383_at3_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto521_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J359,
                    J360,
                    J383,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J360, __, __, __, __, __, J359, __, __, __, __, __, __, __, J359, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J383, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto356_ctx189_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J359 => goto359_at3_ctx356_x(lex),
                    Jump::J360 => goto360_at3_ctx356_x(lex),
                    Jump::J383 => goto383_at3_ctx356_x(lex),
                    Jump::__ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto424_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto364_at3_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto444_at3_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto495_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J383,
                    J358,
                    J360,
                    J359,
                    J364,
                    J444,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J364, __, J359,
                        J444, __, __, J383, __, __, __, J359, __, __, __, __, __, J383, __, J383,
                        __, __, __, __, __, J383, __, J360, J359, __, __, __, __, __, __, J358, __,
                        J383, __, __, __, __, __, __, __, __, __, __, __, J383, __, __, __, J383,
                        J358, __, __, __, __, __, __, J383, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto356_ctx189_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J383 => goto383_at3_ctx356_x(lex),
                    Jump::J358 => goto358_at3_ctx356_x(lex),
                    Jump::J360 => goto360_at3_ctx356_x(lex),
                    Jump::J359 => goto359_at3_ctx356_x(lex),
                    Jump::J364 => goto364_at3_ctx356_x(lex),
                    Jump::J444 => goto444_at3_ctx356_x(lex),
                    Jump::__ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto508_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto505_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J358,
                    J383,
                    J360,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J358,
                        __, J360, __, J383, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto356_ctx189_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J358 => goto358_at3_ctx356_x(lex),
                    Jump::J383 => goto383_at3_ctx356_x(lex),
                    Jump::J360 => goto360_at3_ctx356_x(lex),
                    Jump::__ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto523_at1_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J432,
                    J521,
                    J424,
                    J495,
                    J508,
                    J505,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J432, J495, __, __, __, __,
                        J505, __, __, __, __, __, __, J508, J521, J424, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto356_ctx189_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J432 => goto432_at2_ctx356_x(lex),
                    Jump::J521 => goto521_at2_ctx356_x(lex),
                    Jump::J424 => goto424_at2_ctx356_x(lex),
                    Jump::J495 => goto495_at2_ctx356_x(lex),
                    Jump::J508 => goto508_at2_ctx356_x(lex),
                    Jump::J505 => goto505_at2_ctx356_x(lex),
                    Jump::__ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto359_at1_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(2usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto397_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto404_at2_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto356_ctx189_x(lex),
                };
                match byte {
                    byte if pattern8(byte) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto412_at1_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J358,
                    J360,
                    J397,
                    J383,
                    J404,
                    J359,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J360, J383, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J358, J383, __, __, __, __, J397, __, J383,
                        __, __, J404, __, __, J383, J359, __, __, J404, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto356_ctx189_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J358 => goto358_at2_ctx356_x(lex),
                    Jump::J360 => goto360_at2_ctx356_x(lex),
                    Jump::J397 => goto397_at2_ctx356_x(lex),
                    Jump::J383 => goto383_at2_ctx356_x(lex),
                    Jump::J404 => goto404_at2_ctx356_x(lex),
                    Jump::J359 => goto359_at2_ctx356_x(lex),
                    Jump::__ => goto356_ctx189_x(lex),
                }
            }
            #[inline]
            fn goto357_ctx356_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J387,
                    J360,
                    J426,
                    J425,
                    J358,
                    J523,
                    J357,
                    J359,
                    J412,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, J357, J357, J357, J357, J357, J357,
                        J357, J357, J357, J357, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J358, __, J359, __, __,
                        __, J360, J387, J412, __, __, __, __, __, __, __, __, J425, __, __, __, __,
                        J426, J523, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto356_ctx356_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J387 => goto387_at1_ctx356_x(lex),
                    Jump::J360 => goto360_at1_ctx356_x(lex),
                    Jump::J426 => goto426_at1_ctx356_x(lex),
                    Jump::J425 => goto425_at1_ctx356_x(lex),
                    Jump::J358 => goto358_at1_ctx356_x(lex),
                    Jump::J523 => goto523_at1_ctx356_x(lex),
                    Jump::J357 => {
                        lex.bump_unchecked(1usize);
                        goto357_ctx356_x(lex)
                    }
                    Jump::J359 => goto359_at1_ctx356_x(lex),
                    Jump::J412 => goto412_at1_ctx356_x(lex),
                    Jump::__ => goto356_ctx356_x(lex),
                }
            }
            #[inline]
            fn goto358_at1<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(2usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto754_ctx754_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Exclamation));
            }
            #[inline]
            fn goto742_ctx754_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::NotEqual));
            }
            #[inline]
            fn goto819_ctx754_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"=") => {
                        lex.bump_unchecked(1usize);
                        goto742_ctx754_x(lex)
                    }
                    _ => goto754_ctx754_x(lex),
                }
            }
            #[inline]
            fn goto359_at1<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(2usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto756_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Dot));
            }
            #[inline]
            fn goto756_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Dot));
            }
            #[inline]
            fn goto592_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto551_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto526_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto528_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto673_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J551,
                    J526,
                    J528,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J526,
                        __, J528, __, J551, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J551 => goto551_at3_ctx188_x(lex),
                    Jump::J526 => goto526_at3_ctx188_x(lex),
                    Jump::J528 => goto528_at3_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto676_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto612_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto527_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto532_at3_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto663_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J526,
                    J612,
                    J527,
                    J532,
                    J551,
                    J528,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J532, __, J527,
                        J612, __, __, J551, __, __, __, J527, __, __, __, __, __, J551, __, J551,
                        __, __, __, __, __, J551, __, J528, J527, __, __, __, __, __, __, J526, __,
                        J551, __, __, __, __, __, __, __, __, __, __, __, J551, __, __, __, J551,
                        J526, __, __, __, __, __, __, J551, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J526 => goto526_at3_ctx188_x(lex),
                    Jump::J612 => goto612_at3_ctx188_x(lex),
                    Jump::J527 => goto527_at3_ctx188_x(lex),
                    Jump::J532 => goto532_at3_ctx188_x(lex),
                    Jump::J551 => goto551_at3_ctx188_x(lex),
                    Jump::J528 => goto528_at3_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto689_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J527,
                    J551,
                    J528,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J528, __, __, __, __, __, J527, __, __, __, __, __, __, __, J527, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J551, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J527 => goto527_at3_ctx188_x(lex),
                    Jump::J551 => goto551_at3_ctx188_x(lex),
                    Jump::J528 => goto528_at3_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto600_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match byte {
                    180u8 => goto527_at3_ctx188_x(lex),
                    146u8 => goto526_at3_ctx188_x(lex),
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto691_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J592,
                    J673,
                    J676,
                    J663,
                    J689,
                    J600,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J600, J663, __, __, __, __,
                        J673, __, __, __, __, __, __, J676, J689, J592, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J592 => goto592_at2_ctx188_x(lex),
                    Jump::J673 => goto673_at2_ctx188_x(lex),
                    Jump::J676 => goto676_at2_ctx188_x(lex),
                    Jump::J663 => goto663_at2_ctx188_x(lex),
                    Jump::J689 => goto689_at2_ctx188_x(lex),
                    Jump::J600 => goto600_at2_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto526_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(2usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto527_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(2usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto551_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto526_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto532_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto555_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J551,
                    J526,
                    J532,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J532, __, J532, __,
                        J532, __, J532, __, J532, __, J532, __, J532, __, J532, __, J532, __, J532,
                        __, J551, __, J551, J526, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J551 => goto551_at2_ctx188_x(lex),
                    Jump::J526 => goto526_at2_ctx188_x(lex),
                    Jump::J532 => goto532_at2_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto527_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto589_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match byte {
                    byte if pattern7(byte) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto528_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto593_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J526,
                    J551,
                    J527,
                    J589,
                    J528,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J526, __, __, __, __, __, __, __, __, __, __, J551, J528, __, __, J589, __,
                        J551, __, __, __, __, __, J527, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J526 => goto526_at2_ctx188_x(lex),
                    Jump::J551 => goto551_at2_ctx188_x(lex),
                    Jump::J527 => goto527_at2_ctx188_x(lex),
                    Jump::J589 => goto589_at2_ctx188_x(lex),
                    Jump::J528 => goto528_at2_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto572_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match byte {
                    byte if pattern8(byte) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto565_at2_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto580_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J526,
                    J572,
                    J551,
                    J527,
                    J565,
                    J528,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J528, J551, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J526, J551, __, __, __, __, J565, __, J551,
                        __, __, J572, __, __, J551, J527, __, __, J572, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto188_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J526 => goto526_at2_ctx188_x(lex),
                    Jump::J572 => goto572_at2_ctx188_x(lex),
                    Jump::J551 => goto551_at2_ctx188_x(lex),
                    Jump::J527 => goto527_at2_ctx188_x(lex),
                    Jump::J565 => goto565_at2_ctx188_x(lex),
                    Jump::J528 => goto528_at2_ctx188_x(lex),
                    Jump::__ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto594_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(1usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto528_at1_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(2usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto188_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto525_ctx188_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J691,
                    J526,
                    J527,
                    J555,
                    J593,
                    J580,
                    J594,
                    J525,
                    J528,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, J525, J525, J525, J525, J525, J525,
                        J525, J525, J525, J525, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J526, __, J527, __, __,
                        __, J528, J555, J580, __, __, __, __, __, __, __, __, J593, __, __, __, __,
                        J594, J691, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto188_ctx188_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J691 => goto691_at1_ctx188_x(lex),
                    Jump::J526 => goto526_at1_ctx188_x(lex),
                    Jump::J527 => goto527_at1_ctx188_x(lex),
                    Jump::J555 => goto555_at1_ctx188_x(lex),
                    Jump::J593 => goto593_at1_ctx188_x(lex),
                    Jump::J580 => goto580_at1_ctx188_x(lex),
                    Jump::J594 => goto594_at1_ctx188_x(lex),
                    Jump::J525 => {
                        lex.bump_unchecked(1usize);
                        goto525_ctx188_x(lex)
                    }
                    Jump::J528 => goto528_at1_ctx188_x(lex),
                    Jump::__ => goto188_ctx188_x(lex),
                }
            }
            #[inline]
            fn goto594_at1_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(1usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto526_at1_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(2usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto592_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto551_at3_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto526_at3_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto528_at3_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto673_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J551,
                    J526,
                    J528,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J526,
                        __, J528, __, J551, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto756_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J551 => goto551_at3_ctx756_x(lex),
                    Jump::J526 => goto526_at3_ctx756_x(lex),
                    Jump::J528 => goto528_at3_ctx756_x(lex),
                    Jump::__ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto676_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto612_at3_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto527_at3_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto532_at3_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(4usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto663_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J526,
                    J612,
                    J527,
                    J532,
                    J551,
                    J528,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J532, __, J527,
                        J612, __, __, J551, __, __, __, J527, __, __, __, __, __, J551, __, J551,
                        __, __, __, __, __, J551, __, J528, J527, __, __, __, __, __, __, J526, __,
                        J551, __, __, __, __, __, __, __, __, __, __, __, J551, __, __, __, J551,
                        J526, __, __, __, __, __, __, J551, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto756_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J526 => goto526_at3_ctx756_x(lex),
                    Jump::J612 => goto612_at3_ctx756_x(lex),
                    Jump::J527 => goto527_at3_ctx756_x(lex),
                    Jump::J532 => goto532_at3_ctx756_x(lex),
                    Jump::J551 => goto551_at3_ctx756_x(lex),
                    Jump::J528 => goto528_at3_ctx756_x(lex),
                    Jump::__ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto689_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J527,
                    J551,
                    J528,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J528, __, __, __, __, __, J527, __, __, __, __, __, __, __, J527, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J551, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto756_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J527 => goto527_at3_ctx756_x(lex),
                    Jump::J551 => goto551_at3_ctx756_x(lex),
                    Jump::J528 => goto528_at3_ctx756_x(lex),
                    Jump::__ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto600_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto756_x(lex),
                };
                match byte {
                    180u8 => goto527_at3_ctx756_x(lex),
                    146u8 => goto526_at3_ctx756_x(lex),
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto691_at1_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J592,
                    J673,
                    J676,
                    J663,
                    J689,
                    J600,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J600, J663, __, __, __, __,
                        J673, __, __, __, __, __, __, J676, J689, J592, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto756_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J592 => goto592_at2_ctx756_x(lex),
                    Jump::J673 => goto673_at2_ctx756_x(lex),
                    Jump::J676 => goto676_at2_ctx756_x(lex),
                    Jump::J663 => goto663_at2_ctx756_x(lex),
                    Jump::J689 => goto689_at2_ctx756_x(lex),
                    Jump::J600 => goto600_at2_ctx756_x(lex),
                    Jump::__ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto527_at1_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(2usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto551_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto526_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto532_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto555_at1_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J551,
                    J526,
                    J532,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J532, __, J532, __,
                        J532, __, J532, __, J532, __, J532, __, J532, __, J532, __, J532, __, J532,
                        __, J551, __, J551, J526, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto756_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J551 => goto551_at2_ctx756_x(lex),
                    Jump::J526 => goto526_at2_ctx756_x(lex),
                    Jump::J532 => goto532_at2_ctx756_x(lex),
                    Jump::__ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto527_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto589_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto756_x(lex),
                };
                match byte {
                    byte if pattern7(byte) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto528_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto593_at1_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J526,
                    J551,
                    J527,
                    J589,
                    J528,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J526, __, __, __, __, __, __, __, __, __, __, J551, J528, __, __, J589, __,
                        J551, __, __, __, __, __, J527, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto756_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J526 => goto526_at2_ctx756_x(lex),
                    Jump::J551 => goto551_at2_ctx756_x(lex),
                    Jump::J527 => goto527_at2_ctx756_x(lex),
                    Jump::J589 => goto589_at2_ctx756_x(lex),
                    Jump::J528 => goto528_at2_ctx756_x(lex),
                    Jump::__ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto572_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto756_x(lex),
                };
                match byte {
                    byte if pattern8(byte) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto565_at2_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(3usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto580_at1_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J526,
                    J572,
                    J551,
                    J527,
                    J565,
                    J528,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J528, J551, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J526, J551, __, __, __, __, J565, __, J551,
                        __, __, J572, __, __, J551, J527, __, __, J572, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto756_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J526 => goto526_at2_ctx756_x(lex),
                    Jump::J572 => goto572_at2_ctx756_x(lex),
                    Jump::J551 => goto551_at2_ctx756_x(lex),
                    Jump::J527 => goto527_at2_ctx756_x(lex),
                    Jump::J565 => goto565_at2_ctx756_x(lex),
                    Jump::J528 => goto528_at2_ctx756_x(lex),
                    Jump::__ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto738_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Range));
            }
            #[inline]
            fn goto528_at1_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(2usize);
                        goto525_ctx188_x(lex)
                    }
                    _ => goto756_x(lex),
                }
            }
            #[inline]
            fn goto821_ctx756_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J594,
                    J526,
                    J691,
                    J527,
                    J555,
                    J593,
                    J580,
                    J738,
                    J525,
                    J528,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J738, __, J525, J525, J525, J525, J525,
                        J525, J525, J525, J525, J525, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J526, __, J527,
                        __, __, __, J528, J555, J580, __, __, __, __, __, __, __, __, J593, __, __,
                        __, __, J594, J691, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto756_ctx756_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J594 => goto594_at1_ctx756_x(lex),
                    Jump::J526 => goto526_at1_ctx756_x(lex),
                    Jump::J691 => goto691_at1_ctx756_x(lex),
                    Jump::J527 => goto527_at1_ctx756_x(lex),
                    Jump::J555 => goto555_at1_ctx756_x(lex),
                    Jump::J593 => goto593_at1_ctx756_x(lex),
                    Jump::J580 => goto580_at1_ctx756_x(lex),
                    Jump::J738 => {
                        lex.bump_unchecked(1usize);
                        goto738_ctx756_x(lex)
                    }
                    Jump::J525 => {
                        lex.bump_unchecked(1usize);
                        goto525_ctx188_x(lex)
                    }
                    Jump::J528 => goto528_at1_ctx756_x(lex),
                    Jump::__ => goto756_ctx756_x(lex),
                }
            }
            #[inline]
            fn goto764_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::LeftBrace));
            }
            #[inline]
            fn goto751_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Plus));
            }
            #[inline]
            fn goto426_at1<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(1usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto760_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::LeftParen));
            }
            #[inline]
            fn goto743_ctx743_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::LessThan));
            }
            #[inline]
            fn goto744_ctx743_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::LessThanOrEqual));
            }
            #[inline]
            fn goto812_ctx743_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"=") => {
                        lex.bump_unchecked(1usize);
                        goto744_ctx743_x(lex)
                    }
                    _ => goto743_ctx743_x(lex),
                }
            }
            #[inline]
            fn goto758_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Colon));
            }
            #[inline]
            fn goto702_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(2usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto700_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(2usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto700_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(3usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto705_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto704_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(3usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto703_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J700,
                    J705,
                    J704,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J704, __, J704, __,
                        J704, __, J704, __, J704, __, J704, __, J704, __, J704, __, J704, __, J704,
                        __, J705, __, J705, J700, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J700 => goto700_at2_ctx4_x(lex),
                    Jump::J705 => goto705_at2_ctx4_x(lex),
                    Jump::J704 => goto704_at2_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto702_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(3usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto710_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    byte if pattern7(byte) => {
                        lex.bump_unchecked(3usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto701_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(3usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto709_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J702,
                    J705,
                    J700,
                    J710,
                    J701,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J700, __, __, __, __, __, __, __, __, __, __, J705, J702, __, __, J710, __,
                        J705, __, __, __, __, __, J701, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J702 => goto702_at2_ctx4_x(lex),
                    Jump::J705 => goto705_at2_ctx4_x(lex),
                    Jump::J700 => goto700_at2_ctx4_x(lex),
                    Jump::J710 => goto710_at2_ctx4_x(lex),
                    Jump::J701 => goto701_at2_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto702_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(4usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto701_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto705_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(4usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto718_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J702,
                    J701,
                    J705,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J702, __, __, __, __, __, J701, __, __, __, __, __, __, __, J701, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J705, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J702 => goto702_at3_ctx4_x(lex),
                    Jump::J701 => goto701_at3_ctx4_x(lex),
                    Jump::J705 => goto705_at3_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto700_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(4usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto713_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    180u8 => goto701_at3_ctx4_x(lex),
                    146u8 => goto700_at3_ctx4_x(lex),
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto716_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J702,
                    J705,
                    J700,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J700,
                        __, J702, __, J705, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J702 => goto702_at3_ctx4_x(lex),
                    Jump::J705 => goto705_at3_ctx4_x(lex),
                    Jump::J700 => goto700_at3_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto719_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto715_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto704_at3_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(4usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto714_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J715,
                    J702,
                    J705,
                    J700,
                    J701,
                    J704,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J704, __, J701,
                        J715, __, __, J705, __, __, __, J701, __, __, __, __, __, J705, __, J705,
                        __, __, __, __, __, J705, __, J702, J701, __, __, __, __, __, __, J700, __,
                        J705, __, __, __, __, __, __, __, __, __, __, __, J705, __, __, __, J705,
                        J700, __, __, __, __, __, __, J705, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J715 => goto715_at3_ctx4_x(lex),
                    Jump::J702 => goto702_at3_ctx4_x(lex),
                    Jump::J705 => goto705_at3_ctx4_x(lex),
                    Jump::J700 => goto700_at3_ctx4_x(lex),
                    Jump::J701 => goto701_at3_ctx4_x(lex),
                    Jump::J704 => goto704_at3_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto717_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto712_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J718,
                    J713,
                    J716,
                    J719,
                    J714,
                    J717,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J713, J714, __, __, __, __,
                        J716, __, __, __, __, __, __, J717, J718, J719, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J718 => goto718_at2_ctx4_x(lex),
                    Jump::J713 => goto713_at2_ctx4_x(lex),
                    Jump::J716 => goto716_at2_ctx4_x(lex),
                    Jump::J719 => goto719_at2_ctx4_x(lex),
                    Jump::J714 => goto714_at2_ctx4_x(lex),
                    Jump::J717 => goto717_at2_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto711_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(1usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto707_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(3usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto708_at2_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    byte if pattern8(byte) => {
                        lex.bump_unchecked(3usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto706_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J707,
                    J702,
                    J705,
                    J700,
                    J708,
                    J701,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J702, J705, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J700, J705, __, __, __, __, J707, __, J705,
                        __, __, J708, __, __, J705, J701, __, __, J708, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J707 => goto707_at2_ctx4_x(lex),
                    Jump::J702 => goto702_at2_ctx4_x(lex),
                    Jump::J705 => goto705_at2_ctx4_x(lex),
                    Jump::J700 => goto700_at2_ctx4_x(lex),
                    Jump::J708 => goto708_at2_ctx4_x(lex),
                    Jump::J701 => goto701_at2_ctx4_x(lex),
                    Jump::__ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto696_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J702,
                    J700,
                    J703,
                    J709,
                    J696,
                    J189,
                    J712,
                    J711,
                    J706,
                    J701,
                    J187,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J189, __, J696, J696, J696, J696, J696,
                        J696, J696, J696, J696, J696, __, __, __, __, __, __, __, __, __, __, __,
                        J187, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, J187, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J700,
                        __, J701, __, __, __, J702, J703, J706, __, __, __, __, __, __, __, __,
                        J709, __, __, __, __, J711, J712, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto4_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J702 => goto702_at1_ctx4_x(lex),
                    Jump::J700 => goto700_at1_ctx4_x(lex),
                    Jump::J703 => goto703_at1_ctx4_x(lex),
                    Jump::J709 => goto709_at1_ctx4_x(lex),
                    Jump::J696 => {
                        lex.bump_unchecked(1usize);
                        goto696_ctx4_x(lex)
                    }
                    Jump::J189 => {
                        lex.bump_unchecked(1usize);
                        goto189_ctx188_x(lex)
                    }
                    Jump::J712 => goto712_at1_ctx4_x(lex),
                    Jump::J711 => goto711_at1_ctx4_x(lex),
                    Jump::J706 => goto706_at1_ctx4_x(lex),
                    Jump::J701 => goto701_at1_ctx4_x(lex),
                    Jump::J187 => goto187_at1_ctx4_x(lex),
                    Jump::__ => goto4_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto701_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(2usize);
                        goto696_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn pattern9(byte: u8) -> bool {
                const LUT: u64 = 35465847073801215u64;
                match 1u64.checked_shl(byte.wrapping_sub(48u8) as u32) {
                    Some(shift) => LUT & shift != 0,
                    None => false,
                }
            }
            #[inline]
            fn goto5_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                while let Some(arr) = lex.read::<&[u8; 16]>() {
                    if pattern9(arr[0]) {
                        if pattern9(arr[1]) {
                            if pattern9(arr[2]) {
                                if pattern9(arr[3]) {
                                    if pattern9(arr[4]) {
                                        if pattern9(arr[5]) {
                                            if pattern9(arr[6]) {
                                                if pattern9(arr[7]) {
                                                    if pattern9(arr[8]) {
                                                        if pattern9(arr[9]) {
                                                            if pattern9(arr[10]) {
                                                                if pattern9(arr[11]) {
                                                                    if pattern9(arr[12]) {
                                                                        if pattern9(arr[13]) {
                                                                            if pattern9(arr[14]) {
                                                                                if pattern9(arr[15])
                                                                                {
                                                                                    lex.bump_unchecked(16);
                                                                                    continue;
                                                                                }
                                                                                lex.bump_unchecked(
                                                                                    15,
                                                                                );
                                                                                return goto4_ctx4_x(lex);
                                                                            }
                                                                            lex.bump_unchecked(14);
                                                                            return goto4_ctx4_x(
                                                                                lex,
                                                                            );
                                                                        }
                                                                        lex.bump_unchecked(13);
                                                                        return goto4_ctx4_x(lex);
                                                                    }
                                                                    lex.bump_unchecked(12);
                                                                    return goto4_ctx4_x(lex);
                                                                }
                                                                lex.bump_unchecked(11);
                                                                return goto4_ctx4_x(lex);
                                                            }
                                                            lex.bump_unchecked(10);
                                                            return goto4_ctx4_x(lex);
                                                        }
                                                        lex.bump_unchecked(9);
                                                        return goto4_ctx4_x(lex);
                                                    }
                                                    lex.bump_unchecked(8);
                                                    return goto4_ctx4_x(lex);
                                                }
                                                lex.bump_unchecked(7);
                                                return goto4_ctx4_x(lex);
                                            }
                                            lex.bump_unchecked(6);
                                            return goto4_ctx4_x(lex);
                                        }
                                        lex.bump_unchecked(5);
                                        return goto4_ctx4_x(lex);
                                    }
                                    lex.bump_unchecked(4);
                                    return goto4_ctx4_x(lex);
                                }
                                lex.bump_unchecked(3);
                                return goto4_ctx4_x(lex);
                            }
                            lex.bump_unchecked(2);
                            return goto4_ctx4_x(lex);
                        }
                        lex.bump_unchecked(1);
                        return goto4_ctx4_x(lex);
                    }
                    return goto4_ctx4_x(lex);
                }
                while lex.test(pattern9) {
                    lex.bump_unchecked(1);
                }
                goto4_ctx4_x(lex);
            }
            #[inline]
            fn goto6_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto4_x(lex),
                };
                match byte {
                    byte if pattern9(byte) => {
                        lex.bump_unchecked(2usize);
                        goto5_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn pattern10(byte: u8) -> bool {
                match byte {
                    b'0'..=b'7' => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto14_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                while let Some(arr) = lex.read::<&[u8; 16]>() {
                    if pattern10(arr[0]) {
                        if pattern10(arr[1]) {
                            if pattern10(arr[2]) {
                                if pattern10(arr[3]) {
                                    if pattern10(arr[4]) {
                                        if pattern10(arr[5]) {
                                            if pattern10(arr[6]) {
                                                if pattern10(arr[7]) {
                                                    if pattern10(arr[8]) {
                                                        if pattern10(arr[9]) {
                                                            if pattern10(arr[10]) {
                                                                if pattern10(arr[11]) {
                                                                    if pattern10(arr[12]) {
                                                                        if pattern10(arr[13]) {
                                                                            if pattern10(arr[14]) {
                                                                                if pattern10(
                                                                                    arr[15],
                                                                                ) {
                                                                                    lex.bump_unchecked(16);
                                                                                    continue;
                                                                                }
                                                                                lex.bump_unchecked(
                                                                                    15,
                                                                                );
                                                                                return goto4_ctx4_x(lex);
                                                                            }
                                                                            lex.bump_unchecked(14);
                                                                            return goto4_ctx4_x(
                                                                                lex,
                                                                            );
                                                                        }
                                                                        lex.bump_unchecked(13);
                                                                        return goto4_ctx4_x(lex);
                                                                    }
                                                                    lex.bump_unchecked(12);
                                                                    return goto4_ctx4_x(lex);
                                                                }
                                                                lex.bump_unchecked(11);
                                                                return goto4_ctx4_x(lex);
                                                            }
                                                            lex.bump_unchecked(10);
                                                            return goto4_ctx4_x(lex);
                                                        }
                                                        lex.bump_unchecked(9);
                                                        return goto4_ctx4_x(lex);
                                                    }
                                                    lex.bump_unchecked(8);
                                                    return goto4_ctx4_x(lex);
                                                }
                                                lex.bump_unchecked(7);
                                                return goto4_ctx4_x(lex);
                                            }
                                            lex.bump_unchecked(6);
                                            return goto4_ctx4_x(lex);
                                        }
                                        lex.bump_unchecked(5);
                                        return goto4_ctx4_x(lex);
                                    }
                                    lex.bump_unchecked(4);
                                    return goto4_ctx4_x(lex);
                                }
                                lex.bump_unchecked(3);
                                return goto4_ctx4_x(lex);
                            }
                            lex.bump_unchecked(2);
                            return goto4_ctx4_x(lex);
                        }
                        lex.bump_unchecked(1);
                        return goto4_ctx4_x(lex);
                    }
                    return goto4_ctx4_x(lex);
                }
                while lex.test(pattern10) {
                    lex.bump_unchecked(1);
                }
                goto4_ctx4_x(lex);
            }
            #[inline]
            fn goto15_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([b'0'..=b'7']) => {
                        lex.bump_unchecked(2usize);
                        goto14_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn pattern11(byte: u8) -> bool {
                match byte {
                    b'0'..=b'1' => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto9_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                while let Some(arr) = lex.read::<&[u8; 16]>() {
                    if pattern11(arr[0]) {
                        if pattern11(arr[1]) {
                            if pattern11(arr[2]) {
                                if pattern11(arr[3]) {
                                    if pattern11(arr[4]) {
                                        if pattern11(arr[5]) {
                                            if pattern11(arr[6]) {
                                                if pattern11(arr[7]) {
                                                    if pattern11(arr[8]) {
                                                        if pattern11(arr[9]) {
                                                            if pattern11(arr[10]) {
                                                                if pattern11(arr[11]) {
                                                                    if pattern11(arr[12]) {
                                                                        if pattern11(arr[13]) {
                                                                            if pattern11(arr[14]) {
                                                                                if pattern11(
                                                                                    arr[15],
                                                                                ) {
                                                                                    lex.bump_unchecked(16);
                                                                                    continue;
                                                                                }
                                                                                lex.bump_unchecked(
                                                                                    15,
                                                                                );
                                                                                return goto4_ctx4_x(lex);
                                                                            }
                                                                            lex.bump_unchecked(14);
                                                                            return goto4_ctx4_x(
                                                                                lex,
                                                                            );
                                                                        }
                                                                        lex.bump_unchecked(13);
                                                                        return goto4_ctx4_x(lex);
                                                                    }
                                                                    lex.bump_unchecked(12);
                                                                    return goto4_ctx4_x(lex);
                                                                }
                                                                lex.bump_unchecked(11);
                                                                return goto4_ctx4_x(lex);
                                                            }
                                                            lex.bump_unchecked(10);
                                                            return goto4_ctx4_x(lex);
                                                        }
                                                        lex.bump_unchecked(9);
                                                        return goto4_ctx4_x(lex);
                                                    }
                                                    lex.bump_unchecked(8);
                                                    return goto4_ctx4_x(lex);
                                                }
                                                lex.bump_unchecked(7);
                                                return goto4_ctx4_x(lex);
                                            }
                                            lex.bump_unchecked(6);
                                            return goto4_ctx4_x(lex);
                                        }
                                        lex.bump_unchecked(5);
                                        return goto4_ctx4_x(lex);
                                    }
                                    lex.bump_unchecked(4);
                                    return goto4_ctx4_x(lex);
                                }
                                lex.bump_unchecked(3);
                                return goto4_ctx4_x(lex);
                            }
                            lex.bump_unchecked(2);
                            return goto4_ctx4_x(lex);
                        }
                        lex.bump_unchecked(1);
                        return goto4_ctx4_x(lex);
                    }
                    return goto4_ctx4_x(lex);
                }
                while lex.test(pattern11) {
                    lex.bump_unchecked(1);
                }
                goto4_ctx4_x(lex);
            }
            #[inline]
            fn goto10_at1_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([b'0'..=b'1']) => {
                        lex.bump_unchecked(2usize);
                        goto9_ctx4_x(lex)
                    }
                    _ => goto4_x(lex),
                }
            }
            #[inline]
            fn goto695_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J701,
                    J702,
                    J6,
                    J700,
                    J703,
                    J709,
                    J696,
                    J189,
                    J712,
                    J15,
                    J711,
                    J706,
                    J187,
                    J10,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J189, __, J696, J696, J696, J696, J696,
                        J696, J696, J696, J696, J696, __, __, __, __, __, __, __, __, J10, __, __,
                        J187, __, __, __, __, __, __, __, __, __, J15, __, __, __, __, __, __, __,
                        __, J6, __, __, __, __, __, __, __, __, __, J10, __, __, J187, __, __, __,
                        __, __, __, __, __, __, J15, __, __, __, __, __, __, __, __, J6, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J700, __, J701, __, __, __, J702, J703, J706, __, __, __, __, __, __, __,
                        __, J709, __, __, __, __, J711, J712, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto4_ctx4_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J701 => goto701_at1_ctx4_x(lex),
                    Jump::J702 => goto702_at1_ctx4_x(lex),
                    Jump::J6 => goto6_at1_ctx4_x(lex),
                    Jump::J700 => goto700_at1_ctx4_x(lex),
                    Jump::J703 => goto703_at1_ctx4_x(lex),
                    Jump::J709 => goto709_at1_ctx4_x(lex),
                    Jump::J696 => {
                        lex.bump_unchecked(1usize);
                        goto696_ctx4_x(lex)
                    }
                    Jump::J189 => {
                        lex.bump_unchecked(1usize);
                        goto189_ctx188_x(lex)
                    }
                    Jump::J712 => goto712_at1_ctx4_x(lex),
                    Jump::J15 => goto15_at1_ctx4_x(lex),
                    Jump::J711 => goto711_at1_ctx4_x(lex),
                    Jump::J706 => goto706_at1_ctx4_x(lex),
                    Jump::J187 => goto187_at1_ctx4_x(lex),
                    Jump::J10 => goto10_at1_ctx4_x(lex),
                    Jump::__ => goto4_ctx4_x(lex),
                }
            }
            #[inline]
            fn goto759_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Semicolon));
            }
            #[inline]
            fn goto749_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Modulo));
            }
            #[inline]
            fn goto775_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::In));
            }
            #[inline]
            fn goto844_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto775_ctx1_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto775_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto771_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::If));
            }
            #[inline]
            fn goto832_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto771_ctx1_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto771_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto781_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Impl));
            }
            #[inline]
            fn goto873_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto781_ctx1_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto781_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto872_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 2usize]>() {
                    Some(b"pl") => {
                        lex.bump_unchecked(2usize);
                        goto873_ctx1_x(lex)
                    }
                    _ => goto2_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto870_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J844,
                    J832,
                    J872,
                    J2,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, J2, J2, J2, J2, J2, J2, J2, J2, J2,
                        J2, __, __, __, __, __, __, __, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2,
                        J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, __, __, __, __,
                        J2, __, J2, J2, J2, J2, J2, J832, J2, J2, J2, J2, J2, J2, J872, J844, J2,
                        J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto1_ctx1_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J844 => {
                        lex.bump_unchecked(1usize);
                        goto844_ctx1_x(lex)
                    }
                    Jump::J832 => {
                        lex.bump_unchecked(1usize);
                        goto832_ctx1_x(lex)
                    }
                    Jump::J872 => {
                        lex.bump_unchecked(1usize);
                        goto872_ctx1_x(lex)
                    }
                    Jump::J2 => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    Jump::__ => goto1_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto755_ctx755_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Question));
            }
            #[inline]
            fn goto747_ctx755_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Coalesce));
            }
            #[inline]
            fn goto820_ctx755_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"?") => {
                        lex.bump_unchecked(1usize);
                        goto747_ctx755_x(lex)
                    }
                    _ => goto755_ctx755_x(lex),
                }
            }
            #[inline]
            fn goto774_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::For));
            }
            #[inline]
            fn goto841_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto774_ctx1_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto774_ctx1_x(lex),
                }
            }
            #[inline]
            fn pattern12(byte: u8) -> bool {
                COMPACT_TABLE_0[byte as usize] & 64 > 0
            }
            #[inline]
            fn goto887_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto1_ctx1_x(lex),
                };
                match byte {
                    b'r' => {
                        lex.bump_unchecked(1usize);
                        goto841_ctx1_x(lex)
                    }
                    byte if pattern12(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto1_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto783_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Fn));
            }
            #[inline]
            fn goto886_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto783_ctx1_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto783_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto884_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J887,
                    J886,
                    J2,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, J2, J2, J2, J2, J2, J2, J2, J2, J2,
                        J2, __, __, __, __, __, __, __, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2,
                        J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, __, __, __, __,
                        J2, __, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J886, J887, J2,
                        J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto1_ctx1_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J887 => {
                        lex.bump_unchecked(1usize);
                        goto887_ctx1_x(lex)
                    }
                    Jump::J886 => {
                        lex.bump_unchecked(1usize);
                        goto886_ctx1_x(lex)
                    }
                    Jump::J2 => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    Jump::__ => goto1_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto784_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Mut));
            }
            #[inline]
            fn goto890_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto784_ctx2_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto784_ctx2_x(lex),
                }
            }
            #[inline]
            fn goto889_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 2usize]>() {
                    Some(b"ut") => {
                        lex.bump_unchecked(2usize);
                        goto890_ctx2_x(lex)
                    }
                    _ => goto2_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto773_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::While));
            }
            #[inline]
            fn goto838_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto773_ctx2_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto773_ctx2_x(lex),
                }
            }
            #[inline]
            fn goto837_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 4usize]>() {
                    Some(b"hile") => {
                        lex.bump_unchecked(4usize);
                        goto838_ctx2_x(lex)
                    }
                    _ => goto2_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto748_ctx748_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Divide));
            }
            #[inline]
            fn goto788_ctx748_x<'s>(lex: &mut Lexer<'s>) {
                block_comment(lex).construct(Token::BlockComment, lex);
            }
            #[inline]
            fn goto785_ctx785_x<'s>(lex: &mut Lexer<'s>) {
                let token = Token::LineComment(lex.slice());
                lex.set(Ok(token));
            }
            #[inline]
            fn goto786_ctx785_x<'s>(lex: &mut Lexer<'s>) {
                while let Some(arr) = lex.read::<&[u8; 16]>() {
                    if pattern1(arr[0]) {
                        if pattern1(arr[1]) {
                            if pattern1(arr[2]) {
                                if pattern1(arr[3]) {
                                    if pattern1(arr[4]) {
                                        if pattern1(arr[5]) {
                                            if pattern1(arr[6]) {
                                                if pattern1(arr[7]) {
                                                    if pattern1(arr[8]) {
                                                        if pattern1(arr[9]) {
                                                            if pattern1(arr[10]) {
                                                                if pattern1(arr[11]) {
                                                                    if pattern1(arr[12]) {
                                                                        if pattern1(arr[13]) {
                                                                            if pattern1(arr[14]) {
                                                                                if pattern1(arr[15])
                                                                                {
                                                                                    lex.bump_unchecked(16);
                                                                                    continue;
                                                                                }
                                                                                lex.bump_unchecked(
                                                                                    15,
                                                                                );
                                                                                return goto785_ctx785_x(lex);
                                                                            }
                                                                            lex.bump_unchecked(14);
                                                                            return goto785_ctx785_x(lex);
                                                                        }
                                                                        lex.bump_unchecked(13);
                                                                        return goto785_ctx785_x(
                                                                            lex,
                                                                        );
                                                                    }
                                                                    lex.bump_unchecked(12);
                                                                    return goto785_ctx785_x(lex);
                                                                }
                                                                lex.bump_unchecked(11);
                                                                return goto785_ctx785_x(lex);
                                                            }
                                                            lex.bump_unchecked(10);
                                                            return goto785_ctx785_x(lex);
                                                        }
                                                        lex.bump_unchecked(9);
                                                        return goto785_ctx785_x(lex);
                                                    }
                                                    lex.bump_unchecked(8);
                                                    return goto785_ctx785_x(lex);
                                                }
                                                lex.bump_unchecked(7);
                                                return goto785_ctx785_x(lex);
                                            }
                                            lex.bump_unchecked(6);
                                            return goto785_ctx785_x(lex);
                                        }
                                        lex.bump_unchecked(5);
                                        return goto785_ctx785_x(lex);
                                    }
                                    lex.bump_unchecked(4);
                                    return goto785_ctx785_x(lex);
                                }
                                lex.bump_unchecked(3);
                                return goto785_ctx785_x(lex);
                            }
                            lex.bump_unchecked(2);
                            return goto785_ctx785_x(lex);
                        }
                        lex.bump_unchecked(1);
                        return goto785_ctx785_x(lex);
                    }
                    return goto785_ctx785_x(lex);
                }
                while lex.test(pattern1) {
                    lex.bump_unchecked(1);
                }
                goto785_ctx785_x(lex);
            }
            #[inline]
            fn goto816_ctx748_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto748_ctx748_x(lex),
                };
                match byte {
                    b'*' => {
                        lex.bump_unchecked(1usize);
                        goto788_ctx748_x(lex)
                    }
                    b'/' => {
                        lex.bump_unchecked(1usize);
                        goto786_ctx785_x(lex)
                    }
                    _ => goto748_ctx748_x(lex),
                }
            }
            #[inline]
            fn goto762_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::LeftBracket));
            }
            #[inline]
            fn goto757_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Comma));
            }
            #[inline]
            fn goto790_ctx790_x<'s>(lex: &mut Lexer<'s>) {
                let token = Token::Space(lex.slice());
                lex.set(Ok(token));
            }
            #[inline]
            fn pattern13(byte: u8) -> bool {
                match byte {
                    9u8..=10u8 | 32u8 => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto791_ctx790_x<'s>(lex: &mut Lexer<'s>) {
                while let Some(arr) = lex.read::<&[u8; 16]>() {
                    if pattern13(arr[0]) {
                        if pattern13(arr[1]) {
                            if pattern13(arr[2]) {
                                if pattern13(arr[3]) {
                                    if pattern13(arr[4]) {
                                        if pattern13(arr[5]) {
                                            if pattern13(arr[6]) {
                                                if pattern13(arr[7]) {
                                                    if pattern13(arr[8]) {
                                                        if pattern13(arr[9]) {
                                                            if pattern13(arr[10]) {
                                                                if pattern13(arr[11]) {
                                                                    if pattern13(arr[12]) {
                                                                        if pattern13(arr[13]) {
                                                                            if pattern13(arr[14]) {
                                                                                if pattern13(
                                                                                    arr[15],
                                                                                ) {
                                                                                    lex.bump_unchecked(16);
                                                                                    continue;
                                                                                }
                                                                                lex.bump_unchecked(
                                                                                    15,
                                                                                );
                                                                                return goto790_ctx790_x(lex);
                                                                            }
                                                                            lex.bump_unchecked(14);
                                                                            return goto790_ctx790_x(lex);
                                                                        }
                                                                        lex.bump_unchecked(13);
                                                                        return goto790_ctx790_x(
                                                                            lex,
                                                                        );
                                                                    }
                                                                    lex.bump_unchecked(12);
                                                                    return goto790_ctx790_x(lex);
                                                                }
                                                                lex.bump_unchecked(11);
                                                                return goto790_ctx790_x(lex);
                                                            }
                                                            lex.bump_unchecked(10);
                                                            return goto790_ctx790_x(lex);
                                                        }
                                                        lex.bump_unchecked(9);
                                                        return goto790_ctx790_x(lex);
                                                    }
                                                    lex.bump_unchecked(8);
                                                    return goto790_ctx790_x(lex);
                                                }
                                                lex.bump_unchecked(7);
                                                return goto790_ctx790_x(lex);
                                            }
                                            lex.bump_unchecked(6);
                                            return goto790_ctx790_x(lex);
                                        }
                                        lex.bump_unchecked(5);
                                        return goto790_ctx790_x(lex);
                                    }
                                    lex.bump_unchecked(4);
                                    return goto790_ctx790_x(lex);
                                }
                                lex.bump_unchecked(3);
                                return goto790_ctx790_x(lex);
                            }
                            lex.bump_unchecked(2);
                            return goto790_ctx790_x(lex);
                        }
                        lex.bump_unchecked(1);
                        return goto790_ctx790_x(lex);
                    }
                    return goto790_ctx790_x(lex);
                }
                while lex.test(pattern13) {
                    lex.bump_unchecked(1);
                }
                goto790_ctx790_x(lex);
            }
            #[inline]
            fn goto745_ctx745_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::GreaterThan));
            }
            #[inline]
            fn goto746_ctx745_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::GreaterThanOrEqual));
            }
            #[inline]
            fn goto814_ctx745_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"=") => {
                        lex.bump_unchecked(1usize);
                        goto746_ctx745_x(lex)
                    }
                    _ => goto745_ctx745_x(lex),
                }
            }
            #[inline]
            fn goto763_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::RightBracket));
            }
            #[inline]
            fn goto770_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Let));
            }
            #[inline]
            fn goto829_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto770_ctx2_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto770_ctx2_x(lex),
                }
            }
            #[inline]
            fn goto828_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 2usize]>() {
                    Some(b"et") => {
                        lex.bump_unchecked(2usize);
                        goto829_ctx2_x(lex)
                    }
                    _ => goto2_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto752_ctx752_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Minus));
            }
            #[inline]
            fn goto767_ctx752_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Arrow));
            }
            #[inline]
            fn goto824_ctx752_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b">") => {
                        lex.bump_unchecked(1usize);
                        goto767_ctx752_x(lex)
                    }
                    _ => goto752_ctx752_x(lex),
                }
            }
            #[inline]
            fn goto768_ctx768_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::VerticalBar));
            }
            #[inline]
            fn goto740_ctx768_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Or));
            }
            #[inline]
            fn goto825_ctx768_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"|") => {
                        lex.bump_unchecked(1usize);
                        goto740_ctx768_x(lex)
                    }
                    _ => goto768_ctx768_x(lex),
                }
            }
            #[inline]
            fn goto766_ctx766_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Assign));
            }
            #[inline]
            fn goto741_ctx766_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Equal));
            }
            #[inline]
            fn goto822_ctx766_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"=") => {
                        lex.bump_unchecked(1usize);
                        goto741_ctx766_x(lex)
                    }
                    _ => goto766_ctx766_x(lex),
                }
            }
            #[inline]
            fn goto765_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::RightBrace));
            }
            #[inline]
            fn goto358_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto360_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto397_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto383_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto404_at2<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    byte if pattern8(byte) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto359_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto412_at1<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J358,
                    J360,
                    J397,
                    J383,
                    J404,
                    J359,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J360, J383, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, J358, J383, __, __, __, __, J397, __, J383,
                        __, __, J404, __, __, J383, J359, __, __, J404, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J358 => goto358_at2(lex),
                    Jump::J360 => goto360_at2(lex),
                    Jump::J397 => goto397_at2(lex),
                    Jump::J383 => goto383_at2(lex),
                    Jump::J404 => goto404_at2(lex),
                    Jump::J359 => goto359_at2(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto364_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto387_at1<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J358,
                    J383,
                    J364,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J364, __, J364, __,
                        J364, __, J364, __, J364, __, J364, __, J364, __, J364, __, J364, __, J364,
                        __, J383, __, J383, J358, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J358 => goto358_at2(lex),
                    Jump::J383 => goto383_at2(lex),
                    Jump::J364 => goto364_at2(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto761_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::RightParen));
            }
            #[inline]
            fn goto753_ctx753_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Star));
            }
            #[inline]
            fn goto750_ctx753_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Power));
            }
            #[inline]
            fn goto818_ctx753_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"*") => {
                        lex.bump_unchecked(1usize);
                        goto750_ctx753_x(lex)
                    }
                    _ => goto753_ctx753_x(lex),
                }
            }
            #[inline]
            fn goto360_at1<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(2usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto777_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Continue));
            }
            #[inline]
            fn goto850_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto777_ctx2_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto777_ctx2_x(lex),
                }
            }
            #[inline]
            fn goto849_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 7usize]>() {
                    Some(b"ontinue") => {
                        lex.bump_unchecked(7usize);
                        goto850_ctx2_x(lex)
                    }
                    _ => goto2_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto421_at2<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    byte if pattern7(byte) => {
                        lex.bump_unchecked(3usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto425_at1<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J358,
                    J360,
                    J421,
                    J383,
                    J359,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J358, __, __, __, __, __, __, __, __, __, __, J383, J360, __, __, J421, __,
                        J383, __, __, __, __, __, J359, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J358 => goto358_at2(lex),
                    Jump::J360 => goto360_at2(lex),
                    Jump::J421 => goto421_at2(lex),
                    Jump::J383 => goto383_at2(lex),
                    Jump::J359 => goto359_at2(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto358_at3<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto359_at3<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto432_at2<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    146u8 => goto358_at3(lex),
                    180u8 => goto359_at3(lex),
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto360_at3<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto383_at3<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto521_at2<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J359,
                    J360,
                    J383,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        J360, __, __, __, __, __, J359, __, __, __, __, __, __, __, J359, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J383, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J359 => goto359_at3(lex),
                    Jump::J360 => goto360_at3(lex),
                    Jump::J383 => goto383_at3(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto424_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto364_at3<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto444_at3<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto495_at2<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J383,
                    J358,
                    J360,
                    J359,
                    J364,
                    J444,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J364, __, J359,
                        J444, __, __, J383, __, __, __, J359, __, __, __, __, __, J383, __, J383,
                        __, __, __, __, __, J383, __, J360, J359, __, __, __, __, __, __, J358, __,
                        J383, __, __, __, __, __, __, __, __, __, __, __, J383, __, __, __, J383,
                        J358, __, __, __, __, __, __, J383, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J383 => goto383_at3(lex),
                    Jump::J358 => goto358_at3(lex),
                    Jump::J360 => goto360_at3(lex),
                    Jump::J359 => goto359_at3(lex),
                    Jump::J364 => goto364_at3(lex),
                    Jump::J444 => goto444_at3(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto508_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto357_ctx356_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto505_at2<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J358,
                    J383,
                    J360,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J358,
                        __, J360, __, J383, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J358 => goto358_at3(lex),
                    Jump::J383 => goto383_at3(lex),
                    Jump::J360 => goto360_at3(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto523_at1<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J432,
                    J521,
                    J424,
                    J495,
                    J508,
                    J505,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, J432, J495, __, __, __, __,
                        J505, __, __, __, __, __, __, J508, J521, J424, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J432 => goto432_at2(lex),
                    Jump::J521 => goto521_at2(lex),
                    Jump::J424 => goto424_at2(lex),
                    Jump::J495 => goto495_at2(lex),
                    Jump::J508 => goto508_at2(lex),
                    Jump::J505 => goto505_at2(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto769_ctx769_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Ampersand));
            }
            #[inline]
            fn goto739_ctx769_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::And));
            }
            #[inline]
            fn goto826_ctx769_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"&") => {
                        lex.bump_unchecked(1usize);
                        goto739_ctx769_x(lex)
                    }
                    _ => goto769_ctx769_x(lex),
                }
            }
            #[inline]
            fn goto779_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(Token::Struct));
            }
            #[inline]
            fn goto858_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto779_ctx2_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    _ => goto779_ctx2_x(lex),
                }
            }
            #[inline]
            fn goto857_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 5usize]>() {
                    Some(b"truct") => {
                        lex.bump_unchecked(5usize);
                        goto858_ctx2_x(lex)
                    }
                    _ => goto2_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto891<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J723,
                    J852,
                    J875,
                    J730,
                    J846,
                    J860,
                    J358,
                    J819,
                    J359,
                    J821,
                    J764,
                    J751,
                    J426,
                    J760,
                    J812,
                    J758,
                    J2,
                    J695,
                    J759,
                    J749,
                    J870,
                    J820,
                    J884,
                    J889,
                    J837,
                    J816,
                    J762,
                    J757,
                    J791,
                    J814,
                    J763,
                    J828,
                    J357,
                    J824,
                    J825,
                    J822,
                    J765,
                    J412,
                    J387,
                    J761,
                    J818,
                    J360,
                    J849,
                    J425,
                    J523,
                    J826,
                    J857,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, J791, J791, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, J791, J819, J723,
                        __, __, J749, J826, J730, J760, J761, J818, J751, J757, J824, J821, J816,
                        J695, J357, J357, J357, J357, J357, J357, J357, J357, J357, J758, J759,
                        J812, J822, J814, J820, __, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2,
                        J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J2, J762, __, J763, __,
                        J2, __, J2, J846, J849, J2, J860, J884, J2, J2, J870, J2, J2, J828, J889,
                        J2, J2, J2, J2, J852, J857, J875, J2, J2, J837, J2, J2, J2, J764, J825,
                        J765, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, J358,
                        __, J359, __, __, __, J360, J387, J412, __, __, __, __, __, __, __, __,
                        J425, __, __, __, __, J426, J523, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return _end(lex),
                };
                match LUT[byte as usize] {
                    Jump::J723 => {
                        lex.bump_unchecked(1usize);
                        goto723_ctx722_x(lex)
                    }
                    Jump::J852 => {
                        lex.bump_unchecked(1usize);
                        goto852_ctx1_x(lex)
                    }
                    Jump::J875 => {
                        lex.bump_unchecked(1usize);
                        goto875_ctx1_x(lex)
                    }
                    Jump::J730 => {
                        lex.bump_unchecked(1usize);
                        goto730_ctx729_x(lex)
                    }
                    Jump::J846 => {
                        lex.bump_unchecked(1usize);
                        goto846_ctx2_x(lex)
                    }
                    Jump::J860 => {
                        lex.bump_unchecked(1usize);
                        goto860_ctx1_x(lex)
                    }
                    Jump::J358 => goto358_at1(lex),
                    Jump::J819 => {
                        lex.bump_unchecked(1usize);
                        goto819_ctx754_x(lex)
                    }
                    Jump::J359 => goto359_at1(lex),
                    Jump::J821 => {
                        lex.bump_unchecked(1usize);
                        goto821_ctx756_x(lex)
                    }
                    Jump::J764 => {
                        lex.bump_unchecked(1usize);
                        goto764_x(lex)
                    }
                    Jump::J751 => {
                        lex.bump_unchecked(1usize);
                        goto751_x(lex)
                    }
                    Jump::J426 => goto426_at1(lex),
                    Jump::J760 => {
                        lex.bump_unchecked(1usize);
                        goto760_x(lex)
                    }
                    Jump::J812 => {
                        lex.bump_unchecked(1usize);
                        goto812_ctx743_x(lex)
                    }
                    Jump::J758 => {
                        lex.bump_unchecked(1usize);
                        goto758_x(lex)
                    }
                    Jump::J2 => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    Jump::J695 => {
                        lex.bump_unchecked(1usize);
                        goto695_ctx4_x(lex)
                    }
                    Jump::J759 => {
                        lex.bump_unchecked(1usize);
                        goto759_x(lex)
                    }
                    Jump::J749 => {
                        lex.bump_unchecked(1usize);
                        goto749_x(lex)
                    }
                    Jump::J870 => {
                        lex.bump_unchecked(1usize);
                        goto870_ctx1_x(lex)
                    }
                    Jump::J820 => {
                        lex.bump_unchecked(1usize);
                        goto820_ctx755_x(lex)
                    }
                    Jump::J884 => {
                        lex.bump_unchecked(1usize);
                        goto884_ctx1_x(lex)
                    }
                    Jump::J889 => {
                        lex.bump_unchecked(1usize);
                        goto889_ctx2_x(lex)
                    }
                    Jump::J837 => {
                        lex.bump_unchecked(1usize);
                        goto837_ctx2_x(lex)
                    }
                    Jump::J816 => {
                        lex.bump_unchecked(1usize);
                        goto816_ctx748_x(lex)
                    }
                    Jump::J762 => {
                        lex.bump_unchecked(1usize);
                        goto762_x(lex)
                    }
                    Jump::J757 => {
                        lex.bump_unchecked(1usize);
                        goto757_x(lex)
                    }
                    Jump::J791 => {
                        lex.bump_unchecked(1usize);
                        goto791_ctx790_x(lex)
                    }
                    Jump::J814 => {
                        lex.bump_unchecked(1usize);
                        goto814_ctx745_x(lex)
                    }
                    Jump::J763 => {
                        lex.bump_unchecked(1usize);
                        goto763_x(lex)
                    }
                    Jump::J828 => {
                        lex.bump_unchecked(1usize);
                        goto828_ctx2_x(lex)
                    }
                    Jump::J357 => {
                        lex.bump_unchecked(1usize);
                        goto357_ctx356_x(lex)
                    }
                    Jump::J824 => {
                        lex.bump_unchecked(1usize);
                        goto824_ctx752_x(lex)
                    }
                    Jump::J825 => {
                        lex.bump_unchecked(1usize);
                        goto825_ctx768_x(lex)
                    }
                    Jump::J822 => {
                        lex.bump_unchecked(1usize);
                        goto822_ctx766_x(lex)
                    }
                    Jump::J765 => {
                        lex.bump_unchecked(1usize);
                        goto765_x(lex)
                    }
                    Jump::J412 => goto412_at1(lex),
                    Jump::J387 => goto387_at1(lex),
                    Jump::J761 => {
                        lex.bump_unchecked(1usize);
                        goto761_x(lex)
                    }
                    Jump::J818 => {
                        lex.bump_unchecked(1usize);
                        goto818_ctx753_x(lex)
                    }
                    Jump::J360 => goto360_at1(lex),
                    Jump::J849 => {
                        lex.bump_unchecked(1usize);
                        goto849_ctx2_x(lex)
                    }
                    Jump::J425 => goto425_at1(lex),
                    Jump::J523 => goto523_at1(lex),
                    Jump::J826 => {
                        lex.bump_unchecked(1usize);
                        goto826_ctx769_x(lex)
                    }
                    Jump::J857 => {
                        lex.bump_unchecked(1usize);
                        goto857_ctx2_x(lex)
                    }
                    Jump::__ => _error(lex),
                }
            }
            goto891(lex)
        }
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Token<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Token::Ident(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Ident", &__self_0)
                }
                Token::Number(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Number", &__self_0)
                }
                Token::DoubleQuoteString(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DoubleQuoteString",
                        &__self_0,
                    )
                }
                Token::SingleQuoteString(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SingleQuoteString",
                        &__self_0,
                    )
                }
                Token::RawString(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "RawString", &__self_0)
                }
                Token::Boolean(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Boolean", &__self_0)
                }
                Token::Range => ::core::fmt::Formatter::write_str(f, "Range"),
                Token::And => ::core::fmt::Formatter::write_str(f, "And"),
                Token::Or => ::core::fmt::Formatter::write_str(f, "Or"),
                Token::Equal => ::core::fmt::Formatter::write_str(f, "Equal"),
                Token::NotEqual => ::core::fmt::Formatter::write_str(f, "NotEqual"),
                Token::LessThan => ::core::fmt::Formatter::write_str(f, "LessThan"),
                Token::LessThanOrEqual => ::core::fmt::Formatter::write_str(f, "LessThanOrEqual"),
                Token::GreaterThan => ::core::fmt::Formatter::write_str(f, "GreaterThan"),
                Token::GreaterThanOrEqual => {
                    ::core::fmt::Formatter::write_str(f, "GreaterThanOrEqual")
                }
                Token::Coalesce => ::core::fmt::Formatter::write_str(f, "Coalesce"),
                Token::Divide => ::core::fmt::Formatter::write_str(f, "Divide"),
                Token::Modulo => ::core::fmt::Formatter::write_str(f, "Modulo"),
                Token::Power => ::core::fmt::Formatter::write_str(f, "Power"),
                Token::Plus => ::core::fmt::Formatter::write_str(f, "Plus"),
                Token::Minus => ::core::fmt::Formatter::write_str(f, "Minus"),
                Token::Star => ::core::fmt::Formatter::write_str(f, "Star"),
                Token::Exclamation => ::core::fmt::Formatter::write_str(f, "Exclamation"),
                Token::Question => ::core::fmt::Formatter::write_str(f, "Question"),
                Token::Dot => ::core::fmt::Formatter::write_str(f, "Dot"),
                Token::Comma => ::core::fmt::Formatter::write_str(f, "Comma"),
                Token::Colon => ::core::fmt::Formatter::write_str(f, "Colon"),
                Token::Semicolon => ::core::fmt::Formatter::write_str(f, "Semicolon"),
                Token::LeftParen => ::core::fmt::Formatter::write_str(f, "LeftParen"),
                Token::RightParen => ::core::fmt::Formatter::write_str(f, "RightParen"),
                Token::LeftBracket => ::core::fmt::Formatter::write_str(f, "LeftBracket"),
                Token::RightBracket => ::core::fmt::Formatter::write_str(f, "RightBracket"),
                Token::LeftBrace => ::core::fmt::Formatter::write_str(f, "LeftBrace"),
                Token::RightBrace => ::core::fmt::Formatter::write_str(f, "RightBrace"),
                Token::Assign => ::core::fmt::Formatter::write_str(f, "Assign"),
                Token::Arrow => ::core::fmt::Formatter::write_str(f, "Arrow"),
                Token::VerticalBar => ::core::fmt::Formatter::write_str(f, "VerticalBar"),
                Token::Ampersand => ::core::fmt::Formatter::write_str(f, "Ampersand"),
                Token::Let => ::core::fmt::Formatter::write_str(f, "Let"),
                Token::If => ::core::fmt::Formatter::write_str(f, "If"),
                Token::Else => ::core::fmt::Formatter::write_str(f, "Else"),
                Token::While => ::core::fmt::Formatter::write_str(f, "While"),
                Token::For => ::core::fmt::Formatter::write_str(f, "For"),
                Token::In => ::core::fmt::Formatter::write_str(f, "In"),
                Token::Break => ::core::fmt::Formatter::write_str(f, "Break"),
                Token::Continue => ::core::fmt::Formatter::write_str(f, "Continue"),
                Token::Return => ::core::fmt::Formatter::write_str(f, "Return"),
                Token::Struct => ::core::fmt::Formatter::write_str(f, "Struct"),
                Token::Enum => ::core::fmt::Formatter::write_str(f, "Enum"),
                Token::Impl => ::core::fmt::Formatter::write_str(f, "Impl"),
                Token::Type => ::core::fmt::Formatter::write_str(f, "Type"),
                Token::Fn => ::core::fmt::Formatter::write_str(f, "Fn"),
                Token::Mut => ::core::fmt::Formatter::write_str(f, "Mut"),
                Token::LineComment(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "LineComment", &__self_0)
                }
                Token::BlockComment(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "BlockComment", &__self_0)
                }
                Token::Space(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Space", &__self_0)
                }
                Token::EOF => ::core::fmt::Formatter::write_str(f, "EOF"),
                Token::Error => ::core::fmt::Formatter::write_str(f, "Error"),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for Token<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for Token<'a> {
        #[inline]
        fn eq(&self, other: &Token<'a>) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (Token::Ident(__self_0), Token::Ident(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Token::Number(__self_0), Token::Number(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Token::DoubleQuoteString(__self_0), Token::DoubleQuoteString(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Token::SingleQuoteString(__self_0), Token::SingleQuoteString(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Token::RawString(__self_0), Token::RawString(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Token::Boolean(__self_0), Token::Boolean(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Token::LineComment(__self_0), Token::LineComment(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Token::BlockComment(__self_0), Token::BlockComment(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Token::Space(__self_0), Token::Space(__arg1_0)) => *__self_0 == *__arg1_0,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for Token<'a> {
        #[inline]
        fn clone(&self) -> Token<'a> {
            match self {
                Token::Ident(__self_0) => Token::Ident(::core::clone::Clone::clone(__self_0)),
                Token::Number(__self_0) => Token::Number(::core::clone::Clone::clone(__self_0)),
                Token::DoubleQuoteString(__self_0) => {
                    Token::DoubleQuoteString(::core::clone::Clone::clone(__self_0))
                }
                Token::SingleQuoteString(__self_0) => {
                    Token::SingleQuoteString(::core::clone::Clone::clone(__self_0))
                }
                Token::RawString(__self_0) => {
                    Token::RawString(::core::clone::Clone::clone(__self_0))
                }
                Token::Boolean(__self_0) => Token::Boolean(::core::clone::Clone::clone(__self_0)),
                Token::Range => Token::Range,
                Token::And => Token::And,
                Token::Or => Token::Or,
                Token::Equal => Token::Equal,
                Token::NotEqual => Token::NotEqual,
                Token::LessThan => Token::LessThan,
                Token::LessThanOrEqual => Token::LessThanOrEqual,
                Token::GreaterThan => Token::GreaterThan,
                Token::GreaterThanOrEqual => Token::GreaterThanOrEqual,
                Token::Coalesce => Token::Coalesce,
                Token::Divide => Token::Divide,
                Token::Modulo => Token::Modulo,
                Token::Power => Token::Power,
                Token::Plus => Token::Plus,
                Token::Minus => Token::Minus,
                Token::Star => Token::Star,
                Token::Exclamation => Token::Exclamation,
                Token::Question => Token::Question,
                Token::Dot => Token::Dot,
                Token::Comma => Token::Comma,
                Token::Colon => Token::Colon,
                Token::Semicolon => Token::Semicolon,
                Token::LeftParen => Token::LeftParen,
                Token::RightParen => Token::RightParen,
                Token::LeftBracket => Token::LeftBracket,
                Token::RightBracket => Token::RightBracket,
                Token::LeftBrace => Token::LeftBrace,
                Token::RightBrace => Token::RightBrace,
                Token::Assign => Token::Assign,
                Token::Arrow => Token::Arrow,
                Token::VerticalBar => Token::VerticalBar,
                Token::Ampersand => Token::Ampersand,
                Token::Let => Token::Let,
                Token::If => Token::If,
                Token::Else => Token::Else,
                Token::While => Token::While,
                Token::For => Token::For,
                Token::In => Token::In,
                Token::Break => Token::Break,
                Token::Continue => Token::Continue,
                Token::Return => Token::Return,
                Token::Struct => Token::Struct,
                Token::Enum => Token::Enum,
                Token::Impl => Token::Impl,
                Token::Type => Token::Type,
                Token::Fn => Token::Fn,
                Token::Mut => Token::Mut,
                Token::LineComment(__self_0) => {
                    Token::LineComment(::core::clone::Clone::clone(__self_0))
                }
                Token::BlockComment(__self_0) => {
                    Token::BlockComment(::core::clone::Clone::clone(__self_0))
                }
                Token::Space(__self_0) => Token::Space(::core::clone::Clone::clone(__self_0)),
                Token::EOF => Token::EOF,
                Token::Error => Token::Error,
            }
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub enum TokenKind {
        Ident,
        Number,
        DoubleQuoteString,
        SingleQuoteString,
        RawString,
        Boolean,
        Range,
        And,
        Or,
        Equal,
        NotEqual,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
        Coalesce,
        Divide,
        Modulo,
        Power,
        Plus,
        Minus,
        Star,
        Exclamation,
        Question,
        Dot,
        Comma,
        Colon,
        Semicolon,
        LeftParen,
        RightParen,
        LeftBracket,
        RightBracket,
        LeftBrace,
        RightBrace,
        Assign,
        Arrow,
        VerticalBar,
        Ampersand,
        Let,
        If,
        Else,
        While,
        For,
        In,
        Break,
        Continue,
        Return,
        Struct,
        Enum,
        Impl,
        Type,
        Fn,
        Mut,
        LineComment,
        BlockComment,
        Space,
        EOF,
        Error,
    }
    #[automatically_derived]
    #[allow(dead_code)]
    #[allow(missing_docs)]
    impl ::core::fmt::Debug for TokenKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    TokenKind::Ident => "Ident",
                    TokenKind::Number => "Number",
                    TokenKind::DoubleQuoteString => "DoubleQuoteString",
                    TokenKind::SingleQuoteString => "SingleQuoteString",
                    TokenKind::RawString => "RawString",
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
                    TokenKind::Plus => "Plus",
                    TokenKind::Minus => "Minus",
                    TokenKind::Star => "Star",
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
                    TokenKind::Assign => "Assign",
                    TokenKind::Arrow => "Arrow",
                    TokenKind::VerticalBar => "VerticalBar",
                    TokenKind::Ampersand => "Ampersand",
                    TokenKind::Let => "Let",
                    TokenKind::If => "If",
                    TokenKind::Else => "Else",
                    TokenKind::While => "While",
                    TokenKind::For => "For",
                    TokenKind::In => "In",
                    TokenKind::Break => "Break",
                    TokenKind::Continue => "Continue",
                    TokenKind::Return => "Return",
                    TokenKind::Struct => "Struct",
                    TokenKind::Enum => "Enum",
                    TokenKind::Impl => "Impl",
                    TokenKind::Type => "Type",
                    TokenKind::Fn => "Fn",
                    TokenKind::Mut => "Mut",
                    TokenKind::LineComment => "LineComment",
                    TokenKind::BlockComment => "BlockComment",
                    TokenKind::Space => "Space",
                    TokenKind::EOF => "EOF",
                    TokenKind::Error => "Error",
                },
            )
        }
    }
    #[automatically_derived]
    #[allow(dead_code)]
    #[allow(missing_docs)]
    impl ::core::clone::Clone for TokenKind {
        #[inline]
        fn clone(&self) -> TokenKind {
            *self
        }
    }
    #[automatically_derived]
    #[allow(dead_code)]
    #[allow(missing_docs)]
    impl ::core::marker::Copy for TokenKind {}
    #[allow(dead_code)]
    #[allow(missing_docs)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TokenKind {}
    #[automatically_derived]
    #[allow(dead_code)]
    #[allow(missing_docs)]
    impl ::core::cmp::PartialEq for TokenKind {
        #[inline]
        fn eq(&self, other: &TokenKind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    #[automatically_derived]
    impl ::core::marker::StructuralEq for TokenKind {}
    #[automatically_derived]
    #[allow(dead_code)]
    #[allow(missing_docs)]
    impl ::core::cmp::Eq for TokenKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    #[allow(unused_attributes)]
    impl<'__enum_kinds1, 'a> ::std::convert::From<&'__enum_kinds1 Token<'a>> for TokenKind {
        fn from(_value: &'__enum_kinds1 Token<'a>) -> Self {
            match _value {
                &Token::Ident(..) => TokenKind::Ident,
                &Token::Number(..) => TokenKind::Number,
                &Token::DoubleQuoteString(..) => TokenKind::DoubleQuoteString,
                &Token::SingleQuoteString(..) => TokenKind::SingleQuoteString,
                &Token::RawString(..) => TokenKind::RawString,
                &Token::Boolean(..) => TokenKind::Boolean,
                &Token::Range => TokenKind::Range,
                &Token::And => TokenKind::And,
                &Token::Or => TokenKind::Or,
                &Token::Equal => TokenKind::Equal,
                &Token::NotEqual => TokenKind::NotEqual,
                &Token::LessThan => TokenKind::LessThan,
                &Token::LessThanOrEqual => TokenKind::LessThanOrEqual,
                &Token::GreaterThan => TokenKind::GreaterThan,
                &Token::GreaterThanOrEqual => TokenKind::GreaterThanOrEqual,
                &Token::Coalesce => TokenKind::Coalesce,
                &Token::Divide => TokenKind::Divide,
                &Token::Modulo => TokenKind::Modulo,
                &Token::Power => TokenKind::Power,
                &Token::Plus => TokenKind::Plus,
                &Token::Minus => TokenKind::Minus,
                &Token::Star => TokenKind::Star,
                &Token::Exclamation => TokenKind::Exclamation,
                &Token::Question => TokenKind::Question,
                &Token::Dot => TokenKind::Dot,
                &Token::Comma => TokenKind::Comma,
                &Token::Colon => TokenKind::Colon,
                &Token::Semicolon => TokenKind::Semicolon,
                &Token::LeftParen => TokenKind::LeftParen,
                &Token::RightParen => TokenKind::RightParen,
                &Token::LeftBracket => TokenKind::LeftBracket,
                &Token::RightBracket => TokenKind::RightBracket,
                &Token::LeftBrace => TokenKind::LeftBrace,
                &Token::RightBrace => TokenKind::RightBrace,
                &Token::Assign => TokenKind::Assign,
                &Token::Arrow => TokenKind::Arrow,
                &Token::VerticalBar => TokenKind::VerticalBar,
                &Token::Ampersand => TokenKind::Ampersand,
                &Token::Let => TokenKind::Let,
                &Token::If => TokenKind::If,
                &Token::Else => TokenKind::Else,
                &Token::While => TokenKind::While,
                &Token::For => TokenKind::For,
                &Token::In => TokenKind::In,
                &Token::Break => TokenKind::Break,
                &Token::Continue => TokenKind::Continue,
                &Token::Return => TokenKind::Return,
                &Token::Struct => TokenKind::Struct,
                &Token::Enum => TokenKind::Enum,
                &Token::Impl => TokenKind::Impl,
                &Token::Type => TokenKind::Type,
                &Token::Fn => TokenKind::Fn,
                &Token::Mut => TokenKind::Mut,
                &Token::LineComment(..) => TokenKind::LineComment,
                &Token::BlockComment(..) => TokenKind::BlockComment,
                &Token::Space(..) => TokenKind::Space,
                &Token::EOF => TokenKind::EOF,
                &Token::Error => TokenKind::Error,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_attributes)]
    impl<'__enum_kinds1, 'a> ::std::convert::From<Token<'a>> for TokenKind {
        fn from(value: Token<'a>) -> Self {
            TokenKind::from(&value)
        }
    }
    pub fn parse_Ident<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, &'token_parser_a str> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Ident(data) => Ok((input, data)),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Ident]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Number<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, Number> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match &first.token {
            Token::Number(data) => Ok((input, data.clone())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Number]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_DoubleQuoteString<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, &'token_parser_a str> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::DoubleQuoteString(data) => Ok((input, data)),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::DoubleQuoteString]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_SingleQuoteString<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, &'token_parser_a str> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::SingleQuoteString(data) => Ok((input, data)),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::SingleQuoteString]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_RawString<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, &'token_parser_a str> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::RawString(data) => Ok((input, data)),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::RawString]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Boolean<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, bool> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Boolean(data) => Ok((input, data)),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Boolean]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Range<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Range => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Range]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_And<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::And => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::And]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Or<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Or => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Or]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Equal<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Equal => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Equal]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_NotEqual<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::NotEqual => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::NotEqual]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_LessThan<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::LessThan => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::LessThan]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_LessThanOrEqual<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::LessThanOrEqual => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::LessThanOrEqual]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_GreaterThan<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::GreaterThan => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::GreaterThan]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_GreaterThanOrEqual<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::GreaterThanOrEqual => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::GreaterThanOrEqual]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Coalesce<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Coalesce => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Coalesce]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Divide<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Divide => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Divide]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Modulo<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Modulo => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Modulo]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Power<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Power => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Power]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Plus<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Plus => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Plus]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Minus<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Minus => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Minus]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Star<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Star => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Star]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Exclamation<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Exclamation => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Exclamation]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Question<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Question => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Question]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Dot<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Dot => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Dot]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Comma<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Comma => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Comma]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Colon<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Colon => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Colon]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Semicolon<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Semicolon => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Semicolon]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_LeftParen<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::LeftParen => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::LeftParen]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_RightParen<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::RightParen => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::RightParen]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_LeftBracket<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::LeftBracket => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::LeftBracket]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_RightBracket<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::RightBracket => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::RightBracket]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_LeftBrace<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::LeftBrace => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::LeftBrace]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_RightBrace<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::RightBrace => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::RightBrace]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Assign<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Assign => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Assign]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Arrow<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Arrow => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Arrow]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_VerticalBar<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::VerticalBar => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::VerticalBar]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Ampersand<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Ampersand => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Ampersand]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Let<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Let => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Let]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_If<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::If => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::If]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Else<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Else => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Else]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_While<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::While => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::While]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_For<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::For => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::For]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_In<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::In => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::In]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Break<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Break => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Break]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Continue<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Continue => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Continue]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Return<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Return => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Return]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Struct<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Struct => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Struct]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Enum<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Enum => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Enum]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Impl<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Impl => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Impl]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Type<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Type => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Type]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Fn<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Fn => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Fn]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Mut<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Mut => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Mut]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_LineComment<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, &'token_parser_a str> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::LineComment(data) => Ok((input, data)),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::LineComment]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_BlockComment<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, &'token_parser_a str> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::BlockComment(data) => Ok((input, data)),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::BlockComment]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Space<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, &'token_parser_a str> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Space(data) => Ok((input, data)),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Space]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_EOF<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::EOF => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::EOF]),
                ),
            )
            .locate(start)),
        }
    }
    pub fn parse_Error<'token_parser_a, 'token_parser_b>(
        input: &'token_parser_b Span<'token_parser_a>,
    ) -> ParserResult<'token_parser_a, ()> {
        let start = input.start;
        let (input, first_span) = input.take_n_token(1)?;
        let first = &first_span.tokens[0];
        match first.token {
            Token::Error => Ok((input, ())),
            _ => Err(crate::ParserError::UnexpectedToken(
                first.kind(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([TokenKind::Error]),
                ),
            )
            .locate(start)),
        }
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
    }
    fn number<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Number {
        let text = lex.slice();
        if text.starts_with("0") && text.len() > 1 {
            match text.chars().nth(1).unwrap() {
                'x' | 'X' => {
                    return Number::Integer(
                        BigInt::from_str_radix(&text[2..], 16).expect("Invalid hexadecimal number"),
                    );
                }
                'o' | 'O' => {
                    return Number::Integer(
                        BigInt::from_str_radix(&text[2..], 8).expect("Invalid octal number"),
                    );
                }
                'b' | 'B' => {
                    return Number::Integer(
                        BigInt::from_str_radix(&text[2..], 2).expect("Invalid binary number"),
                    );
                }
                _ => {}
            };
        }
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
}
