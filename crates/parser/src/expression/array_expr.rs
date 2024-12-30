use fst::Expression;
use parser_core::*;

use crate::utils::ws0;

use super::parse_expression;

pub fn parse_array_expr(input: Span) -> ParserResult<Expression> {
    let (input, _) = parse_left_bracket(input)?;

    let (input, (values, _)) = separated_list(
        (ws0, parse_comma).tuple(),
        (ws0, parse_expression).tuple().map(|(_, expr)| expr),
        (ws0, parse_right_bracket).tuple(),
        true,
        true,
        false,
    )(input)?;

    Ok((input, Expression::Array { elements: values }))
}

#[cfg(test)]
mod tests {
    use crate::{expression::literal_expr::parse_literal_expr, utils::ParseString};

    use super::*;

    #[test]
    fn test_parse_array_expr() {
        let input = "[1, 2 + 3, 3]";
        let expected = Expression::Array {
            elements: vec![
                parse_literal_expr.parse_string("1").unwrap(),
                parse_expression.parse_string("2 + 3").unwrap(),
                parse_literal_expr.parse_string("3").unwrap(),
            ],
        };
        let result = parse_array_expr.parse_string(input).unwrap();
        assert_eq!(expected, result);
    }
}
