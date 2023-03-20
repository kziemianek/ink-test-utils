#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod foobar {

    #[ink(storage)]
    pub struct Foobar {}

    #[ink(event)]
    pub struct FoobarHappened {}

    #[ink(event)]
    pub struct BarfooHappened {}

    impl Foobar {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn foobar(&self) {
            self.env().emit_event(FoobarHappened {});
            self.env().emit_event(BarfooHappened {});
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::foobar::Foobar;
        use ink_test_utils::assert_events_order;

        type Event = <Foobar as ::ink::reflect::ContractEventBase>::Type;

        #[ink::test]
        pub fn test_single_assertion() {
            let foobar = Foobar::new();
            foobar.foobar();
            assert_events_order! {
               [FoobarHappened, BarfooHappened]
            }
        }
    }
}
