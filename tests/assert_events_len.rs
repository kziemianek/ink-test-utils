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
        use crate::foobar::Foobar;
        use ink_test_utils::assert_events_len;

        #[ink::test]
        pub fn test() {
            let foobar = Foobar::new();
            foobar.foobar();
            assert_events_len!(1)
        }
    }
}
