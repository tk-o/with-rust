use std::collections::HashMap;

pub trait MarketConfig {
    type ProviderId;
    type Provider;
    type MarketerId;
    type Marketer;
    type BuyerId;
    type Buyer;
    type SupplyId;
    type Supply: UpdateState<SupplyState>;
    type Transaction;
    type Advertisement;
    type State = MarketState<Self>;

    fn state(&self) -> MarketState<Self>;

    fn state_mut(&mut self) -> &mut MarketState<Self>;
}

trait UpdateState<State> {
    fn set_state(&mut self, state: State);
}

pub enum SupplyState {
    Created,
    Marketed,
    Consumed,
}

pub struct MarketState<T: MarketConfig> {
    providers: HashMap<T::SupplyId, T::Provider>,
    marketers: HashMap<T::MarketerId, T::Marketer>,
    buyers: HashMap<T::BuyerId, T::Buyer>,
    supplies: HashMap<T::SupplyId, T::Supply>,
}

impl<T: MarketConfig> MarketState<T> {
    fn new() -> Self {
        Self {
            providers: HashMap::new(),
            marketers: HashMap::new(),
            buyers: HashMap::new(),
            supplies: HashMap::new(),
        }
    }

    fn buy(&mut self, buyer: &T::Buyer, ad: &T::Advertisement) -> Option<T::Transaction> {
        let supply = match self.supplies.get_mut(&ad.supply) {
            None => return None,
            Some(value) => value,
        };

        let buyer = match self.buyers.get(&buyer.id) {
            None => return None,
            Some(value) => value,
        };

        supply.set_state(T::SupplyState::Consumed);

        Some(T::Transaction::new(buyer.id.clone(), ad.to_owned()))
    }

    fn advertise(&self, supply: &mut T::Supply) -> Option<T::Advertisement> {
        if self.marketers.get(&T::MarketerId("m1".into())).is_none() {
            return None;
        };

        if supply.has_supply_available() == false {
            return None;
        }

        // make the state transition to be exectued
        supply.set_state(T::SupplyState::Marketed);

        Some(T::Advertisement::new(self.id.clone(), supply.id.clone()))
    }
}