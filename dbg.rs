mod core {
    use crate::{ast::Location, combine_errors, lexer::{Token, TokenKind}};
    mod error_union {}
    use thiserror::Error;
    pub struct Span<'a> {
        pub tokens: &'a [LocatedToken<'a>],
        pub start: Location,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Span<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Span",
                "tokens",
                &self.tokens,
                "start",
                &&self.start,
            )
        }
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for Span<'a> {
        #[inline]
        fn clone(&self) -> Span<'a> {
            Span {
                tokens: ::core::clone::Clone::clone(&self.tokens),
                start: ::core::clone::Clone::clone(&self.start),
            }
        }
    }
    pub struct LocatedToken<'a> {
        pub start: Location,
        pub text: &'a str,
        pub token: Token<'a>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for LocatedToken<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "LocatedToken",
                "start",
                &self.start,
                "text",
                &self.text,
                "token",
                &&self.token,
            )
        }
    }
    impl<'a> LocatedToken<'a> {
        pub fn kind(&self) -> TokenKind {
            self.token.kind()
        }
    }
    pub enum TakeParserError {
        #[error("There weren't enough tokens left to take")]
        EndOfInput,
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for TakeParserError {}
    #[allow(unused_qualifications)]
    impl std::fmt::Display for TakeParserError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                TakeParserError::EndOfInput {} => {
                    __formatter
                        .write_fmt(
                            format_args!("There weren\'t enough tokens left to take"),
                        )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TakeParserError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "EndOfInput")
        }
    }
    pub trait TokensLength {
        fn len(&self) -> usize;
        fn new_lines(&self) -> usize;
        fn column(&self) -> usize;
    }
    impl TokensLength for LocatedToken<'_> {
        fn len(&self) -> usize {
            self.text.len()
        }
        fn new_lines(&self) -> usize {
            self.text.chars().filter(|c| *c == '\n').count()
        }
        fn column(&self) -> usize {
            let final_line = self.text.lines().last().unwrap_or("");
            if self.text.lines().count() == 1 {
                self.start.column + final_line.len()
            } else {
                final_line.len()
            }
        }
    }
    impl TokensLength for Span<'_> {
        fn len(&self) -> usize {
            self.tokens.iter().map(|t| t.len()).sum()
        }
        fn new_lines(&self) -> usize {
            self.tokens.iter().map(|t| t.new_lines()).sum()
        }
        fn column(&self) -> usize {
            self.tokens.last().map(|t| t.column()).unwrap_or(self.start.column)
        }
    }
    pub type ParserResult<'a, O, E> = Result<(Span<'a>, O), E>;
    impl<'a> Span<'a> {
        pub fn new(code: &'a [LocatedToken], location: Location) -> Self {
            Self {
                tokens: code,
                start: location,
            }
        }
        pub fn take_n_token(
            &self,
            n: usize,
        ) -> ParserResult<'a, Span<'a>, TakeParserError> {
            if n == 0 {
                return Ok((
                    Span {
                        tokens: &[],
                        start: self.start.clone(),
                    },
                    Span {
                        tokens: self.tokens,
                        start: self.start.clone(),
                    },
                ));
            }
            if n > self.tokens.len() {
                return Err(TakeParserError::EndOfInput);
            }
            let (chunk, rest) = self.tokens.split_at(n);
            let last_token = chunk.last().unwrap();
            let rest_start = Location {
                line: rest
                    .first()
                    .map(|t| t.start.line)
                    .unwrap_or(
                        self.start.line
                            + chunk.iter().map(|t| t.new_lines()).sum::<usize>(),
                    ),
                column: rest
                    .first()
                    .map(|t| t.start.column)
                    .unwrap_or({
                        if chunk.iter().map(|t| t.new_lines()).sum::<usize>() == 0 {
                            self.start.column
                                + chunk.iter().map(|t| t.len()).sum::<usize>()
                        } else {
                            chunk.last().unwrap().column()
                        }
                    }),
                index: rest
                    .first()
                    .map(|t| t.start.index)
                    .unwrap_or(
                        self.start.index + chunk.iter().map(|t| t.len()).sum::<usize>(),
                    ),
            };
            let chunk_span = Span::new(chunk, self.start.clone());
            let rest_span = Span::new(rest, rest_start.clone());
            Ok((rest_span, chunk_span))
        }
        pub fn take_tokens<const N: usize>(
            &self,
        ) -> ParserResult<'a, &'a [LocatedToken<'a>; N], TakeParserError> {
            let (rest, chunk) = self.take_n_token(N)?;
            let chunk = chunk.tokens;
            let chunk = chunk.try_into().unwrap();
            Ok((rest, chunk))
        }
        pub fn take_token(&self) -> ParserResult<'a, LocatedToken<'a>, TakeParserError> {
            let (rest, chunk) = self.take_tokens::<1>()?;
            Ok((rest, chunk[0]))
        }
        pub fn kind(&self) -> Result<TokenKind, TakeParserError> {
            match self.tokens.first() {
                Some(token) => Ok(token.kind()),
                None => Err(TakeParserError::EndOfInput),
            }
        }
    }
    pub fn take_while<'a, F: Fn(&LocatedToken<'a>) -> bool>(
        predicate: F,
    ) -> impl Fn(&Span<'a>) -> (Span<'a>, Span<'a>) {
        move |span: &Span<'a>| {
            let mut i = 0;
            while i < span.tokens.len() && predicate(&span.tokens[i]) {
                i += 1;
            }
            span.take_n_token(i).unwrap()
        }
    }
    pub fn many0<'a, O, E: Clone>(
        parser: impl Fn(&Span<'a>) -> ParserResult<'a, O, E>,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E> {
        move |input: &Span<'a>| {
            let mut input = input.clone();
            let mut output = ::alloc::vec::Vec::new();
            loop {
                match parser(&input) {
                    Ok((rest, o)) => {
                        input = rest;
                        output.push(o);
                    }
                    Err(_) => break,
                }
            }
            Ok((input, output))
        }
    }
    pub fn many1<'a, O, E: Clone>(
        parser: impl Fn(&Span<'a>) -> ParserResult<'a, O, E>,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E> {
        move |input: &Span<'a>| {
            let mut input = input.clone();
            let mut output = ::alloc::vec::Vec::new();
            loop {
                match parser(&input) {
                    Ok((rest, o)) => {
                        input = rest;
                        output.push(o);
                    }
                    Err(e) => {
                        if output.is_empty() {
                            return Err(e);
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok((input, output))
        }
    }
    pub fn separated_list0<'a, O, E: Clone>(
        separator: impl Fn(&Span<'a>) -> ParserResult<'a, (), E>,
        parser: impl Fn(&Span<'a>) -> ParserResult<'a, O, E>,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E> {
        move |input: &Span<'a>| {
            let mut input = input.clone();
            let mut output = ::alloc::vec::Vec::new();
            loop {
                if let Ok((rest, o)) = parser(&input) {
                    input = rest;
                    output.push(o);
                } else {
                    break;
                }
                if let Ok((rest, _)) = separator(&input) {
                    input = rest;
                } else {
                    break;
                }
            }
            Ok((input, output))
        }
    }
    pub fn separated_list1<'a, O, E>(
        separator: impl Fn(&Span<'a>) -> ParserResult<'a, (), E>,
        parser: impl Fn(&Span<'a>) -> ParserResult<'a, O, E>,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E> {
        move |input: &Span<'a>| {
            let mut input = input.clone();
            let mut output = ::alloc::vec::Vec::new();
            match parser(&input) {
                Ok((rest, o)) => {
                    input = rest;
                    output.push(o);
                }
                Err(e) => return Err(e),
            }
            loop {
                if let Ok((rest, _)) = separator(&input) {
                    input = rest;
                } else {
                    break;
                }
                if let Ok((rest, o)) = parser(&input) {
                    input = rest;
                    output.push(o);
                } else {
                    break;
                }
            }
            Ok((input, output))
        }
    }
    pub fn delimited<'a, O, E: From<E1> + From<E2> + From<E3>, E1, E2, E3>(
        start: impl Fn(&Span<'a>) -> ParserResult<'a, (), E1>,
        end: impl Fn(&Span<'a>) -> ParserResult<'a, (), E2>,
        parser: impl Fn(&Span<'a>) -> ParserResult<'a, O, E3>,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E> {
        move |input: &Span<'a>| {
            let mut input = input.clone();
            let (rest, _) = start(&input)?;
            input = rest;
            let (rest, o) = parser(&input)?;
            input = rest;
            let (rest, _) = end(&input)?;
            input = rest;
            Ok((input, o))
        }
    }
    pub enum TokenParserSubError {
        #[error("The token was not the expected kind")]
        WrongTokenKind,
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for TokenParserSubError {}
    #[allow(unused_qualifications)]
    impl std::fmt::Display for TokenParserSubError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                TokenParserSubError::WrongTokenKind {} => {
                    __formatter
                        .write_fmt(format_args!("The token was not the expected kind"))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TokenParserSubError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "WrongTokenKind")
        }
    }
    use std::error::Error;
    use std::fmt::{self, Display, Formatter};
    pub enum TokenParserError {
        TakeParserError(TakeParserError),
        TokenParserSubError(TokenParserSubError),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TokenParserError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                TokenParserError::TakeParserError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "TakeParserError",
                        &__self_0,
                    )
                }
                TokenParserError::TokenParserSubError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "TokenParserSubError",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl From<TakeParserError> for TokenParserError {
        fn from(err: TakeParserError) -> Self {
            TokenParserError::TakeParserError(err)
        }
    }
    impl From<TokenParserSubError> for TokenParserError {
        fn from(err: TokenParserSubError) -> Self {
            TokenParserError::TokenParserSubError(err)
        }
    }
    impl Display for TokenParserError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                TokenParserError::TakeParserError(err) => {
                    f.write_fmt(format_args!("{0}", err))
                }
                TokenParserError::TokenParserSubError(err) => {
                    f.write_fmt(format_args!("{0}", err))
                }
            }
        }
    }
    impl Error for TokenParserError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                TokenParserError::TakeParserError(err) => Some(err),
                TokenParserError::TokenParserSubError(err) => Some(err),
            }
        }
    }
    pub fn token<'a>(
        token_kind: TokenKind,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, &'a LocatedToken<'a>, TokenParserError> {
        move |input: &Span<'a>| {
            let (input, first_span) = input.take_n_token(1)?;
            let first = &first_span.tokens[0];
            if first.kind() == token_kind {
                Ok((input, first))
            } else {
                Err(TokenParserSubError::WrongTokenKind.into())
            }
        }
    }
    pub trait Alt<'a, O, E> {
        fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E>;
    }
    impl<'a, O, E, P: Fn(&Span) -> ParserResult<'a, O, E>> Alt<'a, O, E> for (P,) {
        fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E> {
            move |input: &Span<'a>| (self.0)(input)
        }
    }
    impl<
        'a,
        Out,
        Err,
        A: Fn(&Span) -> ParserResult<'a, Out, Err>,
        B: Fn(&Span) -> ParserResult<'a, Out, Err>,
        C: Fn(&Span) -> ParserResult<'a, Out, Err>,
        D: Fn(&Span) -> ParserResult<'a, Out, Err>,
        E: Fn(&Span) -> ParserResult<'a, Out, Err>,
        F: Fn(&Span) -> ParserResult<'a, Out, Err>,
        G: Fn(&Span) -> ParserResult<'a, Out, Err>,
        H: Fn(&Span) -> ParserResult<'a, Out, Err>,
        I: Fn(&Span) -> ParserResult<'a, Out, Err>,
        J: Fn(&Span) -> ParserResult<'a, Out, Err>,
        K: Fn(&Span) -> ParserResult<'a, Out, Err>,
        L: Fn(&Span) -> ParserResult<'a, Out, Err>,
        M: Fn(&Span) -> ParserResult<'a, Out, Err>,
        N: Fn(&Span) -> ParserResult<'a, Out, Err>,
        O: Fn(&Span) -> ParserResult<'a, Out, Err>,
        P: Fn(&Span) -> ParserResult<'a, Out, Err>,
    > Alt<'a, Out, Err> for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
        fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out, Err> {
            move |input: &Span<'a>| {
                let mut err = None;
                match (self.0)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.1)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.2)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.3)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.4)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.5)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.6)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.7)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.8)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.9)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.10)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.11)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.12)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.13)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.14)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                match (self.15)(input) {
                    Ok(res) => return Ok(res),
                    Err(e) => {
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                Err(err.unwrap())
            }
        }
    }
    pub trait Tuple<'a, O, E> {
        fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, O, E>;
    }
    impl<
        'a,
        Err,
        A: Fn(&Span<'a>) -> ParserResult<'a, O1, Err>,
        B: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
        C: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
        D: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
        E: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
        F: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
        G: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
        H: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
        I: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
        J: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
        K: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
        L: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O1,
        O2,
        O3,
        O4,
        O5,
        O6,
        O7,
        O8,
        O9,
        O10,
        O11,
        O12,
        O13,
        O14,
        O15,
        O16,
    > Tuple<
        'a,
        (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15, O16),
        Err,
    > for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(
            &Span<'a>,
        ) -> ParserResult<
                'a,
                (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15, O16),
                Err,
            > {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, A) = (self.0)(&input)?;
                input = rest;
                let (rest, B) = (self.1)(&input)?;
                input = rest;
                let (rest, C) = (self.2)(&input)?;
                input = rest;
                let (rest, D) = (self.3)(&input)?;
                input = rest;
                let (rest, E) = (self.4)(&input)?;
                input = rest;
                let (rest, F) = (self.5)(&input)?;
                input = rest;
                let (rest, G) = (self.6)(&input)?;
                input = rest;
                let (rest, H) = (self.7)(&input)?;
                input = rest;
                let (rest, I) = (self.8)(&input)?;
                input = rest;
                let (rest, J) = (self.9)(&input)?;
                input = rest;
                let (rest, K) = (self.10)(&input)?;
                input = rest;
                let (rest, L) = (self.11)(&input)?;
                input = rest;
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        B: Fn(&Span<'a>) -> ParserResult<'a, O2, Err>,
        C: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
        D: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
        E: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
        F: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
        G: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
        H: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
        I: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
        J: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
        K: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
        L: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O2,
        O3,
        O4,
        O5,
        O6,
        O7,
        O8,
        O9,
        O10,
        O11,
        O12,
        O13,
        O14,
        O15,
        O16,
    > Tuple<'a, (O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15, O16), Err>
    for (B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(
            &Span<'a>,
        ) -> ParserResult<
                'a,
                (O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15, O16),
                Err,
            > {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, B) = (self.1)(&input)?;
                input = rest;
                let (rest, C) = (self.2)(&input)?;
                input = rest;
                let (rest, D) = (self.3)(&input)?;
                input = rest;
                let (rest, E) = (self.4)(&input)?;
                input = rest;
                let (rest, F) = (self.5)(&input)?;
                input = rest;
                let (rest, G) = (self.6)(&input)?;
                input = rest;
                let (rest, H) = (self.7)(&input)?;
                input = rest;
                let (rest, I) = (self.8)(&input)?;
                input = rest;
                let (rest, J) = (self.9)(&input)?;
                input = rest;
                let (rest, K) = (self.10)(&input)?;
                input = rest;
                let (rest, L) = (self.11)(&input)?;
                input = rest;
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        C: Fn(&Span<'a>) -> ParserResult<'a, O3, Err>,
        D: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
        E: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
        F: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
        G: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
        H: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
        I: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
        J: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
        K: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
        L: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O3,
        O4,
        O5,
        O6,
        O7,
        O8,
        O9,
        O10,
        O11,
        O12,
        O13,
        O14,
        O15,
        O16,
    > Tuple<'a, (O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15, O16), Err>
    for (C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(
            &Span<'a>,
        ) -> ParserResult<
                'a,
                (O3, O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15, O16),
                Err,
            > {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, C) = (self.2)(&input)?;
                input = rest;
                let (rest, D) = (self.3)(&input)?;
                input = rest;
                let (rest, E) = (self.4)(&input)?;
                input = rest;
                let (rest, F) = (self.5)(&input)?;
                input = rest;
                let (rest, G) = (self.6)(&input)?;
                input = rest;
                let (rest, H) = (self.7)(&input)?;
                input = rest;
                let (rest, I) = (self.8)(&input)?;
                input = rest;
                let (rest, J) = (self.9)(&input)?;
                input = rest;
                let (rest, K) = (self.10)(&input)?;
                input = rest;
                let (rest, L) = (self.11)(&input)?;
                input = rest;
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (C, D, E, F, G, H, I, J, K, L, M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        D: Fn(&Span<'a>) -> ParserResult<'a, O4, Err>,
        E: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
        F: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
        G: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
        H: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
        I: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
        J: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
        K: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
        L: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O4,
        O5,
        O6,
        O7,
        O8,
        O9,
        O10,
        O11,
        O12,
        O13,
        O14,
        O15,
        O16,
    > Tuple<'a, (O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15, O16), Err>
    for (D, E, F, G, H, I, J, K, L, M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(
            &Span<'a>,
        ) -> ParserResult<
                'a,
                (O4, O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15, O16),
                Err,
            > {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, D) = (self.3)(&input)?;
                input = rest;
                let (rest, E) = (self.4)(&input)?;
                input = rest;
                let (rest, F) = (self.5)(&input)?;
                input = rest;
                let (rest, G) = (self.6)(&input)?;
                input = rest;
                let (rest, H) = (self.7)(&input)?;
                input = rest;
                let (rest, I) = (self.8)(&input)?;
                input = rest;
                let (rest, J) = (self.9)(&input)?;
                input = rest;
                let (rest, K) = (self.10)(&input)?;
                input = rest;
                let (rest, L) = (self.11)(&input)?;
                input = rest;
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (D, E, F, G, H, I, J, K, L, M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        E: Fn(&Span<'a>) -> ParserResult<'a, O5, Err>,
        F: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
        G: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
        H: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
        I: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
        J: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
        K: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
        L: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O5,
        O6,
        O7,
        O8,
        O9,
        O10,
        O11,
        O12,
        O13,
        O14,
        O15,
        O16,
    > Tuple<'a, (O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15, O16), Err>
    for (E, F, G, H, I, J, K, L, M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(
            &Span<'a>,
        ) -> ParserResult<
                'a,
                (O5, O6, O7, O8, O9, O10, O11, O12, O13, O14, O15, O16),
                Err,
            > {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, E) = (self.4)(&input)?;
                input = rest;
                let (rest, F) = (self.5)(&input)?;
                input = rest;
                let (rest, G) = (self.6)(&input)?;
                input = rest;
                let (rest, H) = (self.7)(&input)?;
                input = rest;
                let (rest, I) = (self.8)(&input)?;
                input = rest;
                let (rest, J) = (self.9)(&input)?;
                input = rest;
                let (rest, K) = (self.10)(&input)?;
                input = rest;
                let (rest, L) = (self.11)(&input)?;
                input = rest;
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (E, F, G, H, I, J, K, L, M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        F: Fn(&Span<'a>) -> ParserResult<'a, O6, Err>,
        G: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
        H: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
        I: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
        J: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
        K: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
        L: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O6,
        O7,
        O8,
        O9,
        O10,
        O11,
        O12,
        O13,
        O14,
        O15,
        O16,
    > Tuple<'a, (O6, O7, O8, O9, O10, O11, O12, O13, O14, O15, O16), Err>
    for (F, G, H, I, J, K, L, M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(
            &Span<'a>,
        ) -> ParserResult<'a, (O6, O7, O8, O9, O10, O11, O12, O13, O14, O15, O16), Err> {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, F) = (self.5)(&input)?;
                input = rest;
                let (rest, G) = (self.6)(&input)?;
                input = rest;
                let (rest, H) = (self.7)(&input)?;
                input = rest;
                let (rest, I) = (self.8)(&input)?;
                input = rest;
                let (rest, J) = (self.9)(&input)?;
                input = rest;
                let (rest, K) = (self.10)(&input)?;
                input = rest;
                let (rest, L) = (self.11)(&input)?;
                input = rest;
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (F, G, H, I, J, K, L, M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        G: Fn(&Span<'a>) -> ParserResult<'a, O7, Err>,
        H: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
        I: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
        J: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
        K: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
        L: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O7,
        O8,
        O9,
        O10,
        O11,
        O12,
        O13,
        O14,
        O15,
        O16,
    > Tuple<'a, (O7, O8, O9, O10, O11, O12, O13, O14, O15, O16), Err>
    for (G, H, I, J, K, L, M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(
            &Span<'a>,
        ) -> ParserResult<'a, (O7, O8, O9, O10, O11, O12, O13, O14, O15, O16), Err> {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, G) = (self.6)(&input)?;
                input = rest;
                let (rest, H) = (self.7)(&input)?;
                input = rest;
                let (rest, I) = (self.8)(&input)?;
                input = rest;
                let (rest, J) = (self.9)(&input)?;
                input = rest;
                let (rest, K) = (self.10)(&input)?;
                input = rest;
                let (rest, L) = (self.11)(&input)?;
                input = rest;
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (G, H, I, J, K, L, M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        H: Fn(&Span<'a>) -> ParserResult<'a, O8, Err>,
        I: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
        J: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
        K: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
        L: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O8,
        O9,
        O10,
        O11,
        O12,
        O13,
        O14,
        O15,
        O16,
    > Tuple<'a, (O8, O9, O10, O11, O12, O13, O14, O15, O16), Err>
    for (H, I, J, K, L, M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(
            &Span<'a>,
        ) -> ParserResult<'a, (O8, O9, O10, O11, O12, O13, O14, O15, O16), Err> {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, H) = (self.7)(&input)?;
                input = rest;
                let (rest, I) = (self.8)(&input)?;
                input = rest;
                let (rest, J) = (self.9)(&input)?;
                input = rest;
                let (rest, K) = (self.10)(&input)?;
                input = rest;
                let (rest, L) = (self.11)(&input)?;
                input = rest;
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (H, I, J, K, L, M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        I: Fn(&Span<'a>) -> ParserResult<'a, O9, Err>,
        J: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
        K: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
        L: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O9,
        O10,
        O11,
        O12,
        O13,
        O14,
        O15,
        O16,
    > Tuple<'a, (O9, O10, O11, O12, O13, O14, O15, O16), Err>
    for (I, J, K, L, M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(
            &Span<'a>,
        ) -> ParserResult<'a, (O9, O10, O11, O12, O13, O14, O15, O16), Err> {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, I) = (self.8)(&input)?;
                input = rest;
                let (rest, J) = (self.9)(&input)?;
                input = rest;
                let (rest, K) = (self.10)(&input)?;
                input = rest;
                let (rest, L) = (self.11)(&input)?;
                input = rest;
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (I, J, K, L, M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        J: Fn(&Span<'a>) -> ParserResult<'a, O10, Err>,
        K: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
        L: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O10,
        O11,
        O12,
        O13,
        O14,
        O15,
        O16,
    > Tuple<'a, (O10, O11, O12, O13, O14, O15, O16), Err> for (J, K, L, M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(
            &Span<'a>,
        ) -> ParserResult<'a, (O10, O11, O12, O13, O14, O15, O16), Err> {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, J) = (self.9)(&input)?;
                input = rest;
                let (rest, K) = (self.10)(&input)?;
                input = rest;
                let (rest, L) = (self.11)(&input)?;
                input = rest;
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (J, K, L, M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        K: Fn(&Span<'a>) -> ParserResult<'a, O11, Err>,
        L: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O11,
        O12,
        O13,
        O14,
        O15,
        O16,
    > Tuple<'a, (O11, O12, O13, O14, O15, O16), Err> for (K, L, M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(
            &Span<'a>,
        ) -> ParserResult<'a, (O11, O12, O13, O14, O15, O16), Err> {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, K) = (self.10)(&input)?;
                input = rest;
                let (rest, L) = (self.11)(&input)?;
                input = rest;
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (K, L, M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        L: Fn(&Span<'a>) -> ParserResult<'a, O12, Err>,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O12,
        O13,
        O14,
        O15,
        O16,
    > Tuple<'a, (O12, O13, O14, O15, O16), Err> for (L, M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O12, O13, O14, O15, O16), Err> {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, L) = (self.11)(&input)?;
                input = rest;
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (L, M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        M: Fn(&Span<'a>) -> ParserResult<'a, O13, Err>,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O13,
        O14,
        O15,
        O16,
    > Tuple<'a, (O13, O14, O15, O16), Err> for (M, N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O13, O14, O15, O16), Err> {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, M) = (self.12)(&input)?;
                input = rest;
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (M, N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        N: Fn(&Span<'a>) -> ParserResult<'a, O14, Err>,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O14,
        O15,
        O16,
    > Tuple<'a, (O14, O15, O16), Err> for (N, O, P) {
        fn tuple(
            &'a self,
        ) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O14, O15, O16), Err> {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, N) = (self.13)(&input)?;
                input = rest;
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (N, O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        O: Fn(&Span<'a>) -> ParserResult<'a, O15, Err>,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O15,
        O16,
    > Tuple<'a, (O15, O16), Err> for (O, P) {
        fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O15, O16), Err> {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, O) = (self.14)(&input)?;
                input = rest;
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (O, P)))
            }
        }
    }
    impl<
        'a,
        Err,
        P: Fn(&Span<'a>) -> ParserResult<'a, O16, Err>,
        O16,
    > Tuple<'a, (O16,), Err> for (P,) {
        fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, (O16,), Err> {
            move |input: &Span<'a>| {
                let mut input = input.clone();
                let (rest, P) = (self.15)(&input)?;
                input = rest;
                Ok((input, (P,)))
            }
        }
    }
    pub enum MoreThanOneError {
        #[error("The parser returned 0 elements, expected at least 1")]
        ZeroElements,
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for MoreThanOneError {}
    #[allow(unused_qualifications)]
    impl std::fmt::Display for MoreThanOneError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                MoreThanOneError::ZeroElements {} => {
                    __formatter
                        .write_fmt(
                            format_args!(
                                "The parser returned 0 elements, expected at least 1"
                            ),
                        )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MoreThanOneError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "ZeroElements")
        }
    }
    pub fn more_than_one<
        'a,
        O,
        E: From<MoreThanOneError>,
        P: Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E>,
    >(parser: P) -> impl Fn(&Span<'a>) -> ParserResult<'a, Vec<O>, E> {
        move |span: &Span<'a>| {
            let (input, output) = parser(span)?;
            if output.len() != 1 {
                return Err(MoreThanOneError::ZeroElements.into());
            }
            Ok((input, output))
        }
    }
    pub fn map<'a, O, O2, E, P: Fn(&Span<'a>) -> ParserResult<'a, O, E>>(
        parser: P,
        wrapper: impl Fn(O) -> O2,
    ) -> impl Fn(&Span<'a>) -> ParserResult<'a, O2, E> {
        move |span: &Span<'a>| {
            let (input, output) = parser(span)?;
            Ok((input, wrapper(output)))
        }
    }
}
