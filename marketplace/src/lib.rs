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
            id: ProviderId("provider".into()),
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
            id: MarketerId("marketer".into()),
            name: "Marketer name".into(),
        }
    }

    /// Marketers are market makers. They pull the supply from the Providers, and put it on the market.
    fn makes_market(self, supply: Supply) -> Option<Ad> {
        if supply.has_supply_available() == false {
            return None;
        }

        // make the state transition to be exectued
        let supply = supply.set_state(SupplyState::Marketed);

        Some(Ad::new(self.id, supply))
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
            id: BuyerId("buyer".into()),
            name,
        }
    }

    fn bids(self, ad: Ad) -> Transaction {
        // FIXME: update the state of the underlying supply
        // ad.supply.state = SupplyState::Consumed;

        Transaction::new(ad, self.id)
    }
}

type AvailableSupply = u32;
#[derive(Debug)]
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

    fn set_state(mut self, state: SupplyState) -> Self {
        self.state = state;

        self
    }
}

#[derive(Debug)]
enum SupplyState {
    Created,
    Marketed,
    Consumed,
}

#[derive(Debug)]
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
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(version(), VERSION);
    }

    #[test]
    fn it_allows_to_create_transaction_between_market_participants() {
        // let's create a place for actors to connect
        let mut makret = Market::new();

        // now, create actors that will interact with each other
        let provider = Provider::new();
        let marketer = Marketer::new();
        let buyer = Buyer::new("mr buyer".into());

        // put the actors into the place
        makret.providers.insert(provider.id, provider.clone());
        makret.marketers.insert(marketer.id, marketer.clone());
        makret.buyers.insert(buyer.id, buyer.clone());

        // everyone is ready to start
        
        // first, the provider needs to manufacture some goods/services
        // well, let's use some sea treasury
        let supply = vec![
            provider.creates_supply("amber".into(), 20),
            provider.creates_supply("pearl".into(), 5),
            provider.creates_supply("sea shell".into(), 100),
        ];

        // the supply is provided, so marketer can start their part of the job
        let jewelry_ads_listing: Vec<Ad> = supply.into_iter().filter_map(|supply| {
            if supply.available_items < 30 {
                marketer.makes_market(supply)
            } else {
                None
            }
        }).collect();

        // let's use some randomness!
        let mut rng = thread_rng();

        // once supply has been put on the market, it is now advertisment, or in short: an ad
        // the ad can be bid against by a buyer, which in turn creates a transaction
        // between the market maker (the marketer) and the market taker (buyer)
        if let Some(ad) = jewelry_ads_listing.choose(&mut rng) {
            // let transaction = buyer.bids(*ad);
            println!("AD picked from the jewelry listing to be bought: {:?}", ad);
        }
    }
}
