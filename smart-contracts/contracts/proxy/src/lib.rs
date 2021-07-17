#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod proxy {
    use ink_storage::lazy::Lazy;

    use controller::controller_contract::ControllerV1Contract;

    /// A simple ERC-20 contract.
    #[ink(storage)]
    pub struct ProxyContract {
        controller_contract: Lazy<ControllerV1Contract>,
    }

    impl ProxyContract {
        #[ink(constructor)]
        pub fn new(controller_hash_address: Hash) -> Self {
            let version = 1_u32;
            let salt = version.to_le_bytes();

            let total_balance = Self::env().balance();
            let controller_contract = ControllerV1Contract::new(98761234)
                .gas_limit(4000)
                .endowment(total_balance / 4)
                .code_hash(controller_hash_address)
                .salt_bytes(salt)
                .instantiate()
                .expect("failed at instantiating the `ControllerContract` contract");

            Self {
                controller_contract: Lazy::new(controller_contract),
            }
        }

        #[ink(message)]
        pub fn get_fees(&self) -> u32 {
            self.controller_contract.get_fees()
        }
    }
}
