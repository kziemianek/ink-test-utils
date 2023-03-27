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
/// # Example
///```
/// # #![cfg_attr(not(feature = "std"), no_std)]
/// # #[ink::contract]
/// # mod foobar {
/// #
/// #   #[ink(storage)]
/// #   pub struct Foobar {}
/// #
/// #   #[ink(event)]
/// #   pub struct FoobarHappened {
/// #       index: u16,
/// #       len: u16,
/// #   }
/// #
/// #   impl Foobar {
/// #       #[ink(constructor)]
/// #       pub fn new() -> Self {
/// #           Self {}
/// #       }
/// #
/// #       #[ink(message)]
/// #       pub fn foobar(&self) {
/// #           self.env().emit_event(FoobarHappened { index: 2, len: 4 });
/// #       }
/// #   }
/// #
/// #   #[cfg(test)]
/// #   mod tests {
/// #       use super::*;
/// #       use crate::foobar::Foobar;
/// #       use ink_test_utils::assert_event;
/// #
/// #       type Event = <Foobar as ::ink::reflect::ContractEventBase>::Type;
/// #
/// #       #[ink::test]
/// #       pub fn test_single_assertion() {
/// #           let expected_index: u16 = 2;
/// #           let foobar = Foobar::new();
/// #           foobar.foobar();
/// assert_event! {
///     0: FoobarHappened (index) [
///         assert_eq!(
///             expected_index, index,
///             "encountered invalid FoobarHappened.index"
///         ),
///     ]
/// }
/// #       }
/// #   }
/// # }
/// ```
#[proc_macro]
pub fn assert_event(input: TokenStream) -> TokenStream {
    assert_event_impl(input)
}

/// Asserts total number of emitted events
///
/// # Example
/// ```
/// # #![cfg_attr(not(feature = "std"), no_std)]
/// #
/// # #[ink::contract]
/// # mod foobar {
/// #
/// #     #[ink(storage)]
/// #     pub struct Foobar {}
/// #
/// #     #[ink(event)]
/// #     pub struct FoobarHappened {
/// #         index: u16,
/// #     }
/// #
/// #     impl Foobar {
/// #         #[ink(constructor)]
/// #         pub fn new() -> Self {
/// #             Self {}
/// #         }
/// #
/// #         #[ink(message)]
/// #         pub fn foobar(&self) {
/// #             self.env().emit_event(FoobarHappened { index: 2 });
/// #         }
/// #     }
/// #
/// #     #[cfg(test)]
/// #     mod tests {
/// #         use crate::foobar::Foobar;
/// #         use ink_test_utils::assert_events_len;
/// #
/// #         #[ink::test]
/// #         pub fn test() {
/// #             let foobar = Foobar::new();
/// #             foobar.foobar();
/// assert_events_len!(1)
/// #         }
/// #     }
/// # }
/// ```
#[proc_macro]
pub fn assert_events_len(input: TokenStream) -> TokenStream {
    assert_events_len_impl(input)
}

/// Asserts events order
///
/// # Example
/// ```
/// # #![cfg_attr(not(feature = "std"), no_std)]
/// #
/// # #[ink::contract]
/// # mod foobar {
/// #
/// #     #[ink(storage)]
/// #     pub struct Foobar {}
/// #
/// #     #[ink(event)]
/// #     pub struct FoobarHappened {}
/// #
/// #     #[ink(event)]
/// #     pub struct BarfooHappened {}
/// #
/// #     impl Foobar {
/// #         #[ink(constructor)]
/// #         pub fn new() -> Self {
/// #             Self {}
/// #         }
/// #
/// #         #[ink(message)]
/// #         pub fn foobar(&self) {
/// #             self.env().emit_event(FoobarHappened {});
/// #             self.env().emit_event(BarfooHappened {});
/// #         }
/// #     }
/// #
/// #     #[cfg(test)]
/// #     mod tests {
/// #         use super::*;
/// #         use crate::foobar::Foobar;
/// #         use ink_test_utils::assert_events_order;
/// #
/// #         type Event = <Foobar as ::ink::reflect::ContractEventBase>::Type;
/// #
/// #         #[ink::test]
/// #         pub fn test_single_assertion() {
/// #             let foobar = Foobar::new();
/// #             foobar.foobar();
/// assert_events_order! {
///     [FoobarHappened, BarfooHappened]
/// }
/// #         }
/// #     }
/// # }
/// ```
#[proc_macro]
pub fn assert_events_order(input: TokenStream) -> TokenStream {
    assert_events_order_impl(input)
}
