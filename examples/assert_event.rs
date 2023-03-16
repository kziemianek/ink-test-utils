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
        pub fn foobar(&self) {}
    }

    mod tests {
        use super::*;
        use ink_test_utils::assert_event;

        type Event = <Foobar as ::ink::reflect::ContractEventBase>::Type;

        pub fn test_dummy() {
            let expected_index: u16 = 10;
            let index: u16 = 5;
            assert_event! {
                2: FoobarHappened (index) [
                    assert_eq!(
                         expected_index, index,
                        "encountered invalid FoobarHappened.index"
                    )
                ]
            };
        }
    }
}

fn main() {
    println!("This is example")
}
