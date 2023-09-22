use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitInt};
use proc_macro2::Span;
use syn::Ident;

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
                fn tuple(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, (#(#type_names,)*)> {
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
                fn alt(&'a self) -> impl Fn(&Span<'a>) -> ParserResult<'a, Out> {
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