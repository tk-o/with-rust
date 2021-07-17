#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod controller_contract {
    // use super::Controller;

    #[ink(storage)]
    pub struct ControllerV1Contract {
        fees: u32,
    }

    impl ControllerV1Contract {
        #[ink(constructor)]
        pub fn new(fees: u32) -> Self {
            Self { fees }
        }

        #[ink(message)]
        pub fn get_fees(&self) -> u32 {
            self.fees
        }
    }
}
//
// #[ink::trait_definition]
// pub trait Controller {
//     #[ink(constructor)]
//     fn new(fees: u32) -> Self;
//
//     #[ink(message)]
//     fn get_fees(&self) -> u32;
// }