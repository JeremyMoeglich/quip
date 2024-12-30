use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::visit_mut::VisitMut;
use syn::{parse_macro_input, LitInt};
use syn::{Data, DataEnum, DeriveInput, Fields, Ident};

#[proc_macro]
pub fn generate_all_tuple_impls(input: TokenStream) -> TokenStream {
    let n: usize = {
        let lit = parse_macro_input!(input as LitInt);
        lit.base10_parse().unwrap()
    };

    let mut all_tokens = proc_macro2::TokenStream::new();

    for i in 1..=n {
        let type_names = (0..i)
            .map(|j| Ident::new(&format!("O{}", j), Span::call_site()))
            .collect::<Vec<_>>();

        let fn_names = (0..i)
            .map(|j| Ident::new(&format!("f{}", j), Span::call_site()))
            .collect::<Vec<_>>();

        let into_result_names = (0..i)
            .map(|j| Ident::new(&format!("I{}", j), Span::call_site()))
            .collect::<Vec<_>>();

        let indices = (0..i).map(|i| syn::Index::from(i)).collect::<Vec<_>>();

        let tokens = quote! {
            impl<'a, #(#fn_names: Fn(Span<'a>) -> #into_result_names),* , #(#type_names),* , #(#into_result_names: IntoParserResult<'a, #type_names>),* > Tuple<'a, (#(#type_names,)*)> for (#(#fn_names,)*) {
                #[inline]
                fn tuple<'b>(&'b self) -> impl Fn(Span<'a>) -> ParserResult<'a, (#(#type_names,)*)> + 'b {
                    move |mut input: Span<'a>| {
                        #(
                            let (rest, #fn_names) = self.#indices(input).into_parser_result()?;
                            input = rest;
                        )*
                        Ok((input, (#(#fn_names,)*)))
                    }
                }
            }
        };

        all_tokens.extend(tokens);
    }

    all_tokens.into()
}

#[proc_macro]
pub fn generate_all_alt_impls(input: TokenStream) -> TokenStream {
    let n: usize = {
        let lit = parse_macro_input!(input as LitInt);
        lit.base10_parse().unwrap()
    };

    let mut all_tokens = proc_macro2::TokenStream::new();

    for i in 1..=n {
        let fn_names = (0..i)
            .map(|j| Ident::new(&format!("f{}", j), Span::call_site()))
            .collect::<Vec<_>>();

        let into_result_names = (0..i)
            .map(|j| Ident::new(&format!("I{}", j), Span::call_site()))
            .collect::<Vec<_>>();

        let indices = (0..i).map(|i| syn::Index::from(i)).collect::<Vec<_>>();

        let tokens = quote! {
            impl<'a, Out, #(#fn_names: Fn(Span<'a>) -> #into_result_names),*, #(#into_result_names: IntoParserResult<'a, Out>),* > Alt<'a, Out> for (#(#fn_names,)*) {
                #[inline]
                fn alt<'b>(&'b self) -> impl Fn(Span<'a>) -> ParserResult<'a, Out> + 'b {
                    move |input: Span<'a>| {
                        let mut err = None;
                        #(
                            match self.#indices(input).into_parser_result() {
                                Ok(res) => return Ok(res),
                                Err(e) => {
                                    err = match err {
                                        None => Some(e),
                                        Some(last) => Some(last.accumulate(e)),
                                    }
                                }
                            }
                        )*
                        Err(err.unwrap())
                    }
                }
            }
        };

        all_tokens.extend(tokens);
    }

    all_tokens.into()
}

struct ReplaceLifetime;

impl syn::visit_mut::VisitMut for ReplaceLifetime {
    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        if lifetime.ident == "a" {
            lifetime.ident = syn::Ident::new("token_parser_a", lifetime.ident.span());
        }
    }
}

#[proc_macro_derive(TokenParser)]
pub fn token_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut parser_fns = quote! {};

    if let Data::Enum(DataEnum { variants, .. }) = input.data {
        for variant in variants {
            let variant_name = &variant.ident;
            let fn_name = format_ident!("parse_{}", to_snake_case(&variant_name.to_string()));
            match variant.fields {
                Fields::Unit => {
                    // Generate the parser for no-data variants
                    let nodata_parser = quote! {
                        #[inline]
                        pub fn #fn_name<'token_parser_a>(input: Span<'token_parser_a>) -> ParserResult<'token_parser_a, ()> {
                            let (input, (token, source_span)) = input.take_token();
                            match token.delocate() {
                                Some(Token::#variant_name) => Ok((input, ())),
                                _ => Err(token.as_parser_error(TokenKind::#variant_name.into(), source_span)),
                            }
                        }
                    };
                    parser_fns.extend(nodata_parser);
                }
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    // Generate the parser for data variants
                    let mut data_type = fields.unnamed.first().unwrap().ty.clone();
                    ReplaceLifetime.visit_type_mut(&mut data_type);
                    let data_parser = quote! {
                        #[inline]
                        pub fn #fn_name<'token_parser_a>(input: Span<'token_parser_a>) -> ParserResult<'token_parser_a, #data_type> {
                            let (input, (token, source_span)) = input.take_token();
                            match token.delocate() {
                                Some(Token::#variant_name(data)) => Ok((input, data)),
                                _ => Err(token.as_parser_error(TokenKind::#variant_name.into(), source_span)),
                            }
                        }
                    };
                    parser_fns.extend(data_parser);
                }
                _ => panic!("Unsupported enum variant!"),
            }
        }
    } else {
        panic!("This macro only supports enums!");
    }

    parser_fns.into()
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            if !result.is_empty()
                && result.chars().last().unwrap() != '_'
                && chars.peek().map_or(false, |next| next.is_lowercase())
            {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}
