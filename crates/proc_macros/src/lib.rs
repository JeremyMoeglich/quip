use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::visit_mut::VisitMut;
use syn::{parse_macro_input, Generics, LitInt};
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
            .map(|j| Ident::new(&format!("F{}", j), Span::call_site()))
            .collect::<Vec<_>>();

        let indices = (0..i).collect::<Vec<_>>();

        let tokens = quote! {
            impl<'a, #(#fn_names: Fn(&Span<'a>) -> ParserResult<'a, #type_names>),* , #(#type_names,)*> Tuple<'a, (#(#type_names,)*)> for (#(#fn_names,)*) {
                fn tuple<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, (#(#type_names,)*)> + 'b {
                    move |input: &Span<'a>| {
                        let mut input = input.clone();
                        #(
                            let (rest, #fn_names) = self.#indices(&input)?;
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
            .map(|j| Ident::new(&format!("F{}", j), Span::call_site()))
            .collect::<Vec<_>>();

        let indices = (0..i).collect::<Vec<_>>();

        let tokens = quote! {
            impl<'a, Out, #(#fn_names: Fn(&Span<'a>) -> ParserResult<'a, Out>),*> Alt<'a, Out> for (#(#fn_names,)*) {
                fn alt<'b>(&'b self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> + 'b {
                    move |input: &Span<'a>| {
                        let mut err = None;
                        #(
                            match self.#indices(input) {
                                Ok(res) => return Ok(res),
                                Err(e) => {
                                    if err.is_none() {
                                        err = Some(e);
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

    let enum_name = &input.ident;
    let Generics { params, .. } = input.generics;
    let mut parser_fns = quote! {};

    if let Data::Enum(DataEnum { variants, .. }) = input.data {
        for variant in variants {
            let variant_name = &variant.ident;
            match variant.fields {
                Fields::Unit => {
                    // Generate the parser for no-data variants
                    let fn_name = format_ident!("parse_{}", variant_name);
                    let nodata_parser = quote! {
                        pub fn #fn_name<'token_parser_a, 'token_parser_b>(input: &'token_parser_b Span<'token_parser_a>) -> ParserResult<'token_parser_a, ()> {
                            let start = input.start;
                            let (input, first_span) = input.take_n_token(1)?;
                            let first = &first_span.tokens[0];
                            match first.token {
                                Token::#variant_name => Ok((input, ())),
                                _ => Err(crate::ParserError::UnexpectedToken(
                                    first.kind(),
                                    vec![TokenKind::#variant_name],
                                ).locate(start)),
                            }
                        }
                    };
                    parser_fns.extend(nodata_parser);
                }
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    // Generate the parser for data variants
                    let mut data_type = fields.unnamed.first().unwrap().ty.clone();
                    ReplaceLifetime.visit_type_mut(&mut data_type);
                    let fn_name = format_ident!("parse_{}", variant_name);
                    let clone_types = vec!["Number"];
                    let (clone_ref, data_quote) = match clone_types.contains(&variant_name.to_string().as_str()) {
                        true => (quote! { & }, quote! { data.clone() }),
                        false => (quote! {}, quote! { data }),
                    };
                    let data_parser = quote! {
                        pub fn #fn_name<'token_parser_a, 'token_parser_b>(input: &'token_parser_b Span<'token_parser_a>) -> ParserResult<'token_parser_a, #data_type> {
                            let start = input.start;
                            let (input, first_span) = input.take_n_token(1)?;
                            let first = &first_span.tokens[0];
                            match #clone_ref first.token {
                                Token::#variant_name(data) => Ok((input, #data_quote)),
                                _ => Err(crate::ParserError::UnexpectedToken(
                                    first.kind(),
                                    vec![TokenKind::#variant_name],
                                ).locate(start)),
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
