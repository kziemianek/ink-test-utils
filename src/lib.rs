extern crate proc_macro;

use crate::assert_event::assert_event_impl;
use crate::assert_events_len::assert_events_len_impl;
use crate::assert_events_order::assert_events_order_impl;
use proc_macro::TokenStream;

mod assert_event;
mod assert_events_len;
mod assert_events_order;

#[proc_macro]
pub fn assert_event(input: TokenStream) -> TokenStream {
    assert_event_impl(input)
}
#[proc_macro]
pub fn assert_events_len(input: TokenStream) -> TokenStream {
    assert_events_len_impl(input)
}
#[proc_macro]
pub fn assert_events_order(input: TokenStream) -> TokenStream {
    assert_events_order_impl(input)
}
