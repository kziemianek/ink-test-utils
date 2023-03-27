use proc_macro::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{bracketed, parse_macro_input, Token, Type};

struct AssertEventsOrderInput {
    pub events: Punctuated<Type, Token![,]>,
}

impl Parse for AssertEventsOrderInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        bracketed!(content in input);
        let events = Punctuated::parse_terminated(&content)?;
        Ok(AssertEventsOrderInput { events })
    }
}

pub fn assert_events_order_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertEventsOrderInput);
    let asserts: Vec<TokenStream2> = input
        .events
        .iter()
        .map(|event| {
            quote! {
                emitted_event = &emitted_events[idx];
                let decoded_event = <Event as scale::Decode>::decode(&mut &emitted_event.data[..])
                    .expect("encountered invalid contract event data buffer");
                 match decoded_event {
                    __ink_EventBase::#event(_) => {},
                    _ => panic!("expected different event at idx: {}", idx)
                 }
                idx += 1;
            }
        })
        .collect();
    let expanded = quote! {
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        let mut emitted_event: &ink::env::test::EmittedEvent;
        let mut idx: usize = 0;

        #(#asserts);*
    };
    TokenStream::from(expanded)
}
