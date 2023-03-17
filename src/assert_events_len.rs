use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, LitInt};

struct AssertEventsLenInput {
    pub len: LitInt,
}

impl Parse for AssertEventsLenInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let len: LitInt = input.parse()?;
        Ok(AssertEventsLenInput { len })
    }
}

pub fn assert_events_len_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertEventsLenInput);
    let len = input.len;
    let expanded = quote! {
        assert_eq!(ink::env::test::recorded_events().count(), #len)
    };
    TokenStream::from(expanded)
}
