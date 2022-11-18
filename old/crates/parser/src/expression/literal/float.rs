use nom::{
    branch::alt,
    character::complete::char,
    character::complete::{digit0, digit1},
    combinator::opt,
    sequence::tuple,
    IResult,
};

use crate::utils::Span;

pub fn parse_float(input: Span) -> IResult<Span, f64> {
    let (input, (left_digits, _, right_digits, exponent)) = tuple((
        digit1,
        char('.'),
        digit0,
        opt(tuple((
            alt((char('e'), char('E'))),
            opt(alt((char('-'), char('+')))),
            digit1,
        ))),
    ))(input)?;
    let mut value = format!("{}.{}", left_digits.fragment(), right_digits.fragment());
    if let Some((_, sign, exponent)) = exponent {
        let mut exponent = exponent.fragment().to_string();
        if let Some(sign) = sign {
            exponent = format!("{}{}", sign, exponent);
        }
        value = format!("{}e{}", value, exponent);
    }
    Ok((
        input,
        value.parse().expect("Failed to parse float, parser bug"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::new_span;
    use pretty_assertions::assert_eq;

    fn test_fn(input: &str) -> Option<f64> {
        match parse_float(new_span(input)) {
            Ok((input2, value)) => match input2.fragment().is_empty() {
                true => Some(value),
                false => None,
            },
            Err(_) => None,
        }
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(test_fn("23.4"), Some(23.4));
        assert_eq!(test_fn("23.4e-2"), Some(23.4e-2));
        assert_eq!(test_fn("23.4e+2"), Some(23.4e+2));
        assert_eq!(test_fn("23.4e2"), Some(23.4e2));
        assert_eq!(test_fn("23539.4235"), Some(23539.4235));
        assert_eq!(test_fn("32,4"), None);
        assert_eq!(test_fn("32.4.5"), None);
    }
}
