use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{bracketed, parse_macro_input, Token, Type};
use syn::token::Token;

struct AssertEventsOrderInput {
    pub events: Punctuated<Type, Token![,]>,
}

impl Parse for AssertEventsOrderInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut content;
        bracketed!(content in input);
        let events = content.parse_terminated(Type::parse)?;
        Ok(AssertEventsOrderInput {events})
    }
}

pub fn assert_events_order_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertEventsOrderInput);
    let asserts: Vec<TokenStream> = input.events.iter()
        .map(|event| {
            quote! {
                println("input1");
            }
        })
        .collect();
    let expanded = quote!{
        #asserts
    };
    TokenStream::from(expanded)
}

