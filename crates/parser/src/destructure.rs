use fst::{
    ImmutableDestructure, ImmutableDestructureProperty, ImmutableExtract, MutableAlias,
    MutableDestructure, MutableDestructureProperty, MutableExtract,
};
use parser_core::*;

use crate::{separated_list::parser::trailing_separated_list, utils::{opt, opt_bool, ws0}};

/// { mut a, b }
/// { a as mut c, b as d }
/// { mut a.b }
/// { a.{b, mut c} }
/// { a.{b, c} as mut d }
pub fn parse_mutable_destructure<'a>(input: Span<'a>) -> ParserResult<'a, MutableDestructure> {
    let (input, _) = parse_left_brace(input)?;

    let (input, (properties, _)) = trailing_separated_list(
        (ws0, parse_comma).tuple(),
        preceded(ws0, parse_mutable_destructure_property),
        (ws0, parse_right_brace).tuple(),
        true,
        false,
    )(input)?;

    Ok((input, properties))
}

fn parse_mutable_destructure_property<'a>(
    input: Span<'a>,
) -> ParserResult<'a, MutableDestructureProperty> {
    let (start_input, (token, source_span)) = input.take_token();

    match token.delocate() {
        Some(Token::Mut) => {
            // MutablePropertyChain

            let (input, (property_chain, _)) = separated_list(
                (ws0, parse_dot).tuple(),
                preceded(ws0, parse_ident).map(|s| s.to_string()),
                (ws0, any_of(TokenKind::Comma | TokenKind::RightBrace)).tuple(),
                false,
                false,
                true,
            )(input)?;

            Ok((
                input,
                MutableDestructureProperty::MutablePropertyChain { property_chain },
            ))
        }
        Some(Token::Ident(property_name)) => {
            let (input, _) = ws0(start_input);
            let (input, (token, _)) = input.take_token();

            match token.delocate() {
                Some(Token::Dot) => {
                    let (input, _) = ws0(input);
                    let (input, extract) = parse_mutable_extract(input)?;

                    let (input, alias) = opt(|input| {
                        let (input, _) = ws0(input);
                        let as_token_span = input.first_token_span();
                        let (input, _) = parse_as(input)?;
                        let (input, _) = ws0(input);
                        let (input, alias) = parse_mutable_alias(input)?;
                        Ok((input, (as_token_span, alias)))
                    })(input);

                    match alias {
                        Some((as_token_span, alias)) => {
                            let immutable_extract = mutable_extract_to_immutable(extract);

                            match immutable_extract {
                                Some(immutable_extract) => Ok((
                                    input,
                                    MutableDestructureProperty::AliasedSubProperties {
                                        property_name: property_name.to_string(),
                                        extract: immutable_extract,
                                        alias,
                                    },
                                )),
                                None => Err(ParserError::UnexpectedToken(
                                    Some(TokenKind::As),
                                    TokenKind::Comma | TokenKind::RightBrace,
                                )
                                .locate(as_token_span)),
                            }
                        }
                        None => Ok((
                            input,
                            MutableDestructureProperty::UnaliasedSubProperties {
                                property_name: property_name.to_string(),
                                extract,
                            },
                        )),
                    }
                }
                Some(Token::As) => {
                    let (input, _) = ws0(input);
                    let (input, alias) = parse_mutable_alias(input)?;
                    Ok((
                        input,
                        MutableDestructureProperty::Property {
                            property_name: property_name.to_string(),
                            alias: Some(alias),
                        },
                    ))
                }
                _ => Ok((
                    start_input,
                    MutableDestructureProperty::Property {
                        property_name: property_name.to_string(),
                        alias: None,
                    },
                )),
            }
        }
        _ => Err(token.as_parser_error(TokenKind::Mut | TokenKind::Ident, source_span)),
    }
}

fn parse_mutable_alias<'a>(input: Span<'a>) -> ParserResult<'a, MutableAlias> {
    let (input, mutable) = opt_bool((parse_mut, ws0).tuple())(input);

    let (input, alias) = parse_ident(input)?;

    Ok((
        input,
        MutableAlias {
            mutable,
            alias: alias.to_string(),
        },
    ))
}

pub fn parse_mutable_extract<'a>(input: Span<'a>) -> ParserResult<'a, MutableExtract> {
    (
        parse_mutable_destructure.map(|p| MutableExtract::Destructured(p)),
        parse_mutable_destructure_property.map(|p| MutableExtract::DirectProperty(Box::new(p))),
    )
        .alt()(input)
}

pub fn parse_immutable_destructure<'a>(input: Span<'a>) -> ParserResult<'a, ImmutableDestructure> {
    let (input, _) = parse_left_brace(input)?;

    let (input, (properties, _)) = separated_list(
        (ws0, parse_comma).tuple(),
        preceded(ws0, parse_immutable_destructure_property),
        (ws0, parse_right_brace).tuple(),
        true,
        true,
        false,
    )(input)?;

    Ok((input, properties))
}

fn parse_immutable_destructure_property<'a>(
    input: Span<'a>,
) -> ParserResult<'a, ImmutableDestructureProperty> {
    let (input, property_name) = parse_ident(input)?;

    let (input, extract) = opt((ws0, parse_dot, ws0, parse_immutable_extract)
        .tuple()
        .map(|(_, _, _, p)| p))(input);

    let (input, alias) = opt((ws0, parse_as, ws0, parse_ident)
        .tuple()
        .map(|(_, _, _, p)| p.to_string()))(input);

    Ok((
        input,
        ImmutableDestructureProperty {
            property_name: property_name.to_string(),
            extract,
            alias,
        },
    ))
}

pub fn parse_immutable_extract<'a>(input: Span<'a>) -> ParserResult<'a, ImmutableExtract> {
    (
        parse_immutable_destructure.map(|p| ImmutableExtract::Destructured(p)),
        parse_immutable_destructure_property.map(|p| ImmutableExtract::DirectProperty(Box::new(p))),
    )
        .alt()(input)
}

fn mutable_extract_to_immutable<'a>(extract: MutableExtract) -> Option<ImmutableExtract> {
    match extract {
        MutableExtract::Destructured(p) => {
            let mut immutable_properties = Vec::with_capacity(p.len());

            for property in p {
                let immutable_property = mutable_destructure_property_to_immutable(property)?;
                immutable_properties.push(immutable_property);
            }

            Some(ImmutableExtract::Destructured(immutable_properties))
        }
        MutableExtract::DirectProperty(p) => {
            let immutable_property = mutable_destructure_property_to_immutable(*p)?;
            Some(ImmutableExtract::DirectProperty(Box::new(
                immutable_property,
            )))
        }
    }
}

fn mutable_destructure_property_to_immutable<'a>(
    property: MutableDestructureProperty,
) -> Option<ImmutableDestructureProperty> {
    match property {
        MutableDestructureProperty::AliasedSubProperties {
            property_name,
            extract,
            alias,
        } => {
            if alias.mutable {
                return None;
            }

            Some(ImmutableDestructureProperty {
                alias: Some(alias.alias),
                extract: Some(extract),
                property_name,
            })
        }
        MutableDestructureProperty::Property {
            property_name,
            alias,
        } => match alias {
            Some(alias) => {
                if alias.mutable {
                    return None;
                }

                Some(ImmutableDestructureProperty {
                    alias: Some(alias.alias),
                    extract: None,
                    property_name,
                })
            }
            None => Some(ImmutableDestructureProperty {
                alias: None,
                extract: None,
                property_name,
            }),
        },
        MutableDestructureProperty::UnaliasedSubProperties {
            property_name,
            extract,
        } => {
            let immutable_extract = mutable_extract_to_immutable(extract)?;

            Some(ImmutableDestructureProperty {
                alias: None,
                extract: Some(immutable_extract),
                property_name,
            })
        }
        MutableDestructureProperty::MutablePropertyChain { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::ParseString;

    #[test]
    fn test_parse_mutable_destructure_basic() {
        // Test simple mutable destructure
        let input = "{ mut a, b }";
        let result = parse_mutable_destructure.parse_string(input).unwrap();
        assert_eq!(
            result,
            vec![
                MutableDestructureProperty::MutablePropertyChain {
                    property_chain: vec!["a".to_string()],
                },
                MutableDestructureProperty::Property {
                    property_name: "b".to_string(),
                    alias: None,
                },
            ]
        );
    }

    #[test]
    fn test_parse_mutable_destructure_with_alias() {
        // Test destructure with aliasing
        let input = "{ a as mut c, b as d }";
        let result = parse_mutable_destructure.parse_string(input).unwrap();
        assert_eq!(
            result,
            vec![
                MutableDestructureProperty::Property {
                    property_name: "a".to_string(),
                    alias: Some(MutableAlias {
                        mutable: true,
                        alias: "c".to_string(),
                    }),
                },
                MutableDestructureProperty::Property {
                    property_name: "b".to_string(),
                    alias: Some(MutableAlias {
                        mutable: false,
                        alias: "d".to_string(),
                    }),
                },
            ]
        );
    }

    #[test]
    fn test_parse_mutable_destructure_nested() {
        // Test nested destructure
        let input = "{ a.{b, mut c} }";
        let result = parse_mutable_destructure.parse_string(input).unwrap();
        assert_eq!(
            result,
            vec![MutableDestructureProperty::UnaliasedSubProperties {
                property_name: "a".to_string(),
                extract: MutableExtract::Destructured(vec![
                    MutableDestructureProperty::Property {
                        property_name: "b".to_string(),
                        alias: None,
                    },
                    MutableDestructureProperty::MutablePropertyChain {
                        property_chain: vec!["c".to_string()],
                    },
                ]),
            }]
        );
    }

    #[test]
    fn test_parse_mutable_destructure_aliased_sub_properties() {
        // Test aliased sub properties
        let input = "{ a.{b, c} as mut d }";
        let result = parse_mutable_destructure.parse_string(input).unwrap();
        assert_eq!(
            result,
            vec![MutableDestructureProperty::AliasedSubProperties {
                property_name: "a".to_string(),
                extract: ImmutableExtract::Destructured(vec![
                    ImmutableDestructureProperty {
                        property_name: "b".to_string(),
                        extract: None,
                        alias: None,
                    },
                    ImmutableDestructureProperty {
                        property_name: "c".to_string(),
                        extract: None,
                        alias: None,
                    },
                ]),
                alias: MutableAlias {
                    mutable: true,
                    alias: "d".to_string(),
                },
            }]
        );
    }

    #[test]
    fn test_parse_immutable_destructure_basic() {
        // Test basic immutable destructure
        let input = "{ a, b }";
        let result = parse_immutable_destructure.parse_string(input).unwrap();
        assert_eq!(
            result,
            vec![
                ImmutableDestructureProperty {
                    property_name: "a".to_string(),
                    extract: None,
                    alias: None,
                },
                ImmutableDestructureProperty {
                    property_name: "b".to_string(),
                    extract: None,
                    alias: None,
                },
            ]
        );
    }

    #[test]
    fn test_parse_immutable_destructure_with_alias() {
        // Test destructure with alias
        let input = "{ a as x, b as y }";
        let result = parse_immutable_destructure.parse_string(input).unwrap();
        assert_eq!(
            result,
            vec![
                ImmutableDestructureProperty {
                    property_name: "a".to_string(),
                    extract: None,
                    alias: Some("x".to_string()),
                },
                ImmutableDestructureProperty {
                    property_name: "b".to_string(),
                    extract: None,
                    alias: Some("y".to_string()),
                },
            ]
        );
    }

    #[test]
    fn test_parse_immutable_destructure_nested() {
        // Test nested immutable destructure
        let input = "{ a.{b, c} }";
        let result = parse_immutable_destructure.parse_string(input).unwrap();
        assert_eq!(
            result,
            vec![ImmutableDestructureProperty {
                property_name: "a".to_string(),
                extract: Some(ImmutableExtract::Destructured(vec![
                    ImmutableDestructureProperty {
                        property_name: "b".to_string(),
                        extract: None,
                        alias: None,
                    },
                    ImmutableDestructureProperty {
                        property_name: "c".to_string(),
                        extract: None,
                        alias: None,
                    },
                ])),
                alias: None,
            }]
        );
    }
}
