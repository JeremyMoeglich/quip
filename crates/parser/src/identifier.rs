

use crate::core::Span;

pub fn parse_identifier(input: Span) -> IResult<Span, String> {
    let (input, begin) = take_while1(|c: char| c.is_alphabetic() || c == '_')(input)?;
    let (input, rest) = take_while(|c: char| c.is_alphanumeric() || c == '_')(input)?;
    let name = format!("{}{}", begin, rest);
    Ok((input, name))
}
