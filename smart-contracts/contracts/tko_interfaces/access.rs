use ink_lang as ink;
use ink_env::AccountId;

#[ink::trait_definition]
pub trait IsOwnable {
    // fn initialize(&mut self);
    //
    // #[ink(message)]
    // fn owner(&self) -> Option<Address>;
    //
    // #[ink(message)]
    // fn renounce_ownership(&mut self) {
    //     self._only_owner();
    //     self._set_owner(None);
    // }
    //
    // #[ink(message)]
    // fn transfer_ownership(&mut self, new_owner: Address) {
    //     self._only_owner();
    //     self._set_owner(Some(new_owner));
    // }

    /// Tells if the message sender is the owner
    #[ink(message)]
    fn owner(&self) -> AccountId;
    // fn _only_owner(&self) {
    //     match self.owner() {
    //         Some(owner) => assert!(owner == self.message_sender(), "Ownable: caller is not the owner"),
    //         None =>  panic!("Ownable: caller is not the owner")
    //     }
    // }
    //
    // fn _set_owner(&mut self, new_owner: Option<Address>);

    // fn _message_sender(&self) -> Address;
}
