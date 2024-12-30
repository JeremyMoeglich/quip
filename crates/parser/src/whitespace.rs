use enumset::{enum_set_union, EnumSet};
use fst::{SpaceElement, Whitespace0, Whitespace1};
use parser_core::*;
use vec1::{Size0Error, Vec1};

const WHITESPACE_KINDS: EnumSet<TokenKind> = enum_set_union!(
    TokenKind::LineComment,
    TokenKind::BlockComment,
    TokenKind::Space,
);

/*
Right now we return the more expensive Whitespace1 / Whitespace0 types
instead of the more efficient FastWhitespace type.

This is because the AST right now has a 'static lifetime and FastWhitespace
requires a reference to the input.
*/

pub fn ws1<'a>(input: Span<'a>) -> ParserResult<'a, Whitespace1> {
    let (input, span) = many1_span(any_of(WHITESPACE_KINDS))(input)?;
    Ok((input, FastWhitespace::new(span).to_whitespace1().unwrap()))
}

pub fn ws0<'a>(input: Span<'a>) -> SafeParserResult<'a, Whitespace0> {
    let (input, span) = many0_span(any_of(WHITESPACE_KINDS))(input);
    (input, FastWhitespace::new(span).to_whitespace0())
}

#[derive(Debug, Clone)]
pub struct FastWhitespace<'a> {
    span: Span<'a>,
}

fn as_space_element(token: &LocatedToken<'_>) -> Option<SpaceElement> {
    match token.token {
        Token::LineComment(v) => Some(SpaceElement::LineComment(v.to_string())),
        Token::BlockComment(v) => Some(SpaceElement::BlockComment(v.to_string())),
        Token::Space(v) => Some(SpaceElement::Space(v.to_string())),
        _ => None,
    }
}

impl<'a> FastWhitespace<'a> {
    fn new(span: Span<'a>) -> Self {
        Self { span }
    }

    pub fn to_space_elements(&self) -> Vec<SpaceElement> {
        self.span
            .tokens
            .iter()
            .filter_map(|v| as_space_element(v))
            .collect()
    }

    pub fn to_whitespace0(&self) -> Whitespace0 {
        self.to_space_elements()
    }

    pub fn to_whitespace1(&self) -> Result<Whitespace1, Size0Error> {
        Vec1::try_from_vec(self.to_space_elements())
    }
}
