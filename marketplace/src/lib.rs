use std::collections::HashMap;
use std::fmt::{Debug};
use rand::thread_rng;
use rand::seq::SliceRandom;

struct Market {
    providers: HashMap<ProviderId, Provider>,
    marketers: HashMap<MarketerId, Marketer>,
    buyers: HashMap<BuyerId, Buyer>,
}

impl Market {
    fn new() -> Self {
        Market {
            providers: HashMap::new(),
            marketers: HashMap::new(),
            buyers: HashMap::new(),
        }
    }
}

#[derive(Clone,Debug,Hash,PartialEq,Eq)]
struct ProviderId(String);

#[derive(Clone)]
struct Provider {
    id: ProviderId,
    name: String,
}

impl Provider {
    fn new() -> Self {
        Self {
            id: ProviderId("p1".into()),
            name: "Provider name".into(),
        }
    }

    fn creates_supply(&self, name: String, available_items: u32) -> Supply {
        Supply::new(self.id.clone(), name, available_items)
    }
}

#[derive(Clone,Debug,Hash,PartialEq,Eq)]
struct MarketerId(String);

#[derive(Clone)]
struct Marketer {
    id: MarketerId,
    name: String,
}

impl Marketer {
    fn new() -> Self {
        Marketer {
            id: MarketerId("m1".into()),
            name: "Marketer name".into(),
        }
    }

    /// Marketers are market makers. They pull the supply from the Providers, and put it on the market.
    fn makes_market(&self, supply: &mut Supply) -> Option<Ad> {
        if supply.has_supply_available() == false {
            return None;
        }

        // make the state transition to be exectued
        supply.set_state(SupplyState::Marketed);

        Some(Ad::new(self.id.clone(), supply.clone()))
    }
}

#[derive(Clone,Debug,Hash,PartialEq,Eq)]
struct BuyerId(String);

#[derive(Clone)]
struct Buyer {
    id: BuyerId,
    name: String,
}

impl Buyer {
    fn new(name: String) -> Self {
        Buyer {
            id: BuyerId("b1".into()),
            name,
        }
    }

    fn bids(&self, ad: &mut Ad) -> Transaction {
        // FIXME: update the state of the underlying supply
        ad.supply.set_state(SupplyState::Consumed);

        println!("ad supply {:?}", ad.supply);

        Transaction::new(ad.clone(), self.id.clone())
    }
}

type AvailableSupply = u32;
#[derive(Clone,Debug)]
struct Supply {
    provided_by: ProviderId,
    name: String,
    available_items: AvailableSupply,
    state: SupplyState,
}

impl Supply {
    fn new(provider_id: ProviderId, name: String, available_items: AvailableSupply) -> Self {
        Self {
            name,
            available_items,
            provided_by: provider_id,
            state: SupplyState::Created,
        }
    }

    fn has_supply_available(&self) -> bool {
        self.available_items > 0
    }

    fn set_state(&mut self, state: SupplyState) {
        self.state = state;
    }
}

#[derive(Clone,Debug)]
enum SupplyState {
    Created,
    Marketed,
    Consumed,
}

#[derive(Clone,Debug)]
struct Ad {
    marketer: MarketerId,
    supply: Supply,
}

impl Ad {
    fn new(marketer_id: MarketerId, supply: Supply) -> Self {
        Self {
            marketer: marketer_id,
            supply,
        }
    }
}

#[derive(Debug)]
struct Transaction {
    ad: Ad,
    taker: BuyerId,
}

impl Transaction {
    fn new(ad: Ad, buyer_id: BuyerId) -> Self {
        Self {
            ad,
            taker: buyer_id,
        }
    }
}

const VERSION: &'static str = "0.0.1";

pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use std::borrow::{Borrow, BorrowMut};

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(version(), VERSION);
    }

    #[test]
    fn it_allows_to_create_transaction_between_market_participants() {
        // let's create a place for actors to connect
        let mut market = Market::new();

        // now, create actors that will interact with each other
        let provider = Provider::new();
        let marketer = Marketer::new();
        let buyer = Buyer::new("mr buyer".into());

        // put the actors into the place
        market.providers.insert(provider.id.clone(), provider);
        market.marketers.insert(marketer.id.clone(), marketer);
        market.buyers.insert(buyer.id.clone(), buyer);

        // everyone is ready to start
        
        // first, the provider needs to manufacture some goods/services
        if let Some(selected_provider) = market.providers.get(&ProviderId("p1".into())) {
            // well, let's use some sea treasury
            let mut supply = vec![
                selected_provider.creates_supply("amber".into(), 20),
                selected_provider.creates_supply("pearl".into(), 5),
                selected_provider.creates_supply("sea shell".into(), 100),
            ];
    
            // the supply is provided, so marketer can start their part of the job
            let mut jewelry_ads_listing: Vec<Ad> = supply.iter_mut().filter_map(|supply| {
                if supply.available_items > 30 {
                    return None;
                }

                match market.marketers.get(&MarketerId("m1".into())) {
                    Some(marketer) => marketer.makes_market(supply),
                    _ => None,
                }
            }).collect();
    
            // let's use some randomness!
            let mut rng = thread_rng();
    
            // once supply has been put on the market, it is now advertisment, or in short: an ad
            // the ad can be bid against by a buyer, which in turn creates a transaction
            // between the market maker (the marketer) and the market taker (buyer)
            if let Some(ad) = jewelry_ads_listing.choose_mut(&mut rng) {
                if let Some(buyer) = market.buyers.get(&BuyerId("b1".into())) {
                    ad.supply.set_state(SupplyState::Consumed);
                    let transaction = buyer.bids(ad);
                    println!("ADs from the jewelry listing: {:?}", jewelry_ads_listing);
                    println!("TX: {:?}", transaction);
                }
            }

            // FIXME: supply must have the statuses updated accordingly after the transaction from above
            println!("All supply available: {:?}", supply);
        }
    }
}
