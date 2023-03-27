extern crate proc_macro;

use crate::assert_event::assert_event_impl;
use crate::assert_events_len::assert_events_len_impl;
use crate::assert_events_order::assert_events_order_impl;
use proc_macro::TokenStream;

mod assert_event;
mod assert_events_len;
mod assert_events_order;

/// Invoke provided code for selected events
///
/// # Usage
///
/// ```
/// <event_idx>: <event_type> (<event_fields>) {
///     <code>,
/// }
/// ```
///
/// # Example
/// ```
/// assert_event! {
///     0: FoobarHappened (index) [
///         assert_eq!(
///             expected_index, index,
///             "encountered invalid FoobarHappened.index"
///         ),
///     ]
/// }
/// ```
#[proc_macro]
pub fn assert_event(input: TokenStream) -> TokenStream {
    assert_event_impl(input)
}

/// Asserts total number of emitted events
///
/// # Example
/// ```
/// assert_events_len!(1)
/// ```
#[proc_macro]
pub fn assert_events_len(input: TokenStream) -> TokenStream {
    assert_events_len_impl(input)
}

/// Asserts events order
///
/// # Example
/// ```
///assert_events_order! {
///     [FoobarHappened, BarfooHappened]
///}
/// ```
#[proc_macro]
pub fn assert_events_order(input: TokenStream) -> TokenStream {
    assert_events_order_impl(input)
}
