#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod foobar {

    #[ink(storage)]
    pub struct Foobar {}

    #[ink(event)]
    pub struct FoobarHappened {
        index: u16,
    }

    impl Foobar {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn foobar(&self) {
            self.env().emit_event(FoobarHappened { index: 2 });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::foobar::Foobar;
        use ink_test_utils::assert_event;

        type Event = <Foobar as ::ink::reflect::ContractEventBase>::Type;

        #[ink::test]
        pub fn test_dummy() {
            let expected_index: u16 = 2;
            let foobar = Foobar::new();
            foobar.foobar();
            assert_event! {
                0: FoobarHappened (index) [
                    assert_eq!(
                         expected_index, index,
                        "encountered invalid FoobarHappened.index"
                    )
                ]
            }
        }
    }
}
