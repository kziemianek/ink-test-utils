extern crate proc_macro;

use crate::assert_event::assert_event_impl;
use proc_macro::TokenStream;

mod assert_event;

#[proc_macro]
pub fn assert_event(input: TokenStream) -> TokenStream {
    assert_event_impl(input)
}
