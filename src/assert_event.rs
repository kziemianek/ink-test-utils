use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{bracketed, parenthesized, parse_macro_input, Ident, LitInt, Result, Token, Type};

struct AssertEventInput {
    pub index: LitInt,
    pub event_name: Ident,
    pub fields: Punctuated<Type, Token![,]>,
    pub asserts: Punctuated<Type, Token![,]>,
}

impl Parse for AssertEventInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut content;
        let index: LitInt = input.parse()?;
        input.parse::<Token![:]>()?;
        let event_name: Ident = input.parse()?;
        parenthesized!(content in input);
        let fields = Punctuated::parse_terminated(&content)?;
        bracketed!(content in input);
        let asserts = Punctuated::parse_terminated(&content)?;
        Ok(AssertEventInput {
            index,
            event_name,
            fields,
            asserts,
        })
    }
}

pub fn assert_event_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertEventInput);
    let index = input.index;
    let event_name = input.event_name;
    let fields = input.fields;
    let asserts = input.asserts.iter();
    let event_name_str = event_name.to_string();
    let expanded = quote! {
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        let event: &ink::env::test::EmittedEvent = &emitted_events[#index];
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::#event_name(#event_name { #fields, .. }) = decoded_event {
            #(#asserts);*
        } else {
            panic!("encountered unexpected event kind: expected a {} event", #event_name_str)
        }
    };
    TokenStream::from(expanded)
}
