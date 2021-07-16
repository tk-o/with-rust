#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod proxy {
    use ink_storage::{
        lazy::Lazy,
    };

    use controller::{Controller, controller_contract::ControllerV1Contract as ControllerV1Contract};

    /// A simple ERC-20 contract.
    #[ink(storage)]
    pub struct ProxyContract {
        controller_contract: Lazy<ControllerV1Contract>,
    }

    impl ProxyContract {
        #[ink(constructor)]
        pub fn new(controller_hash_address: Hash) -> Self {
            let controller_contract = ControllerV1Contract::new(98761234)
                .code_hash(controller_hash_address)
                .instantiate()
                .expect("failed at instantiating the `ControllerContract` contract");

            Self {
                controller_contract,
            }
        }

        #[ink(message)]
        pub fn call_other_contract(&self) -> u32 {
            self.controller_contract.calculate_fees()
        }
    }
}
