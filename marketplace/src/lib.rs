use std::collections::HashMap;
use std::fmt::{Debug};
use rand::thread_rng;
use rand::seq::SliceRandom;

struct Market {
    sellers: HashMap<SellerId, Seller>,
    marketers: HashMap<MarketerId, Marketer>,
    buyers: HashMap<BuyerId, Buyer>,
}

impl Market {
    fn new() -> Self {
        Market {
            sellers: HashMap::new(),
            marketers: HashMap::new(),
            buyers: HashMap::new(),
        }
    }
}

#[derive(Debug,Hash,PartialEq,Eq)]
struct SellerId(String);
struct Seller {
    id: SellerId,
    name: String,
}

impl Seller {
    fn new() -> Self {
        Self {
            id: SellerId("abc".into()),
            name: "seller name".into(),
        }
    }

    fn creates_supply(&self, available_items: u32) -> Supply {
        Supply::new(self.id, available_items)
    }
}

#[derive(Debug,Hash,PartialEq,Eq)]
struct MarketerId(String);
struct Marketer {
    id: MarketerId,
    name: String,
}

impl Marketer {
    fn new() -> Self {
        Marketer {
            id: MarketerId("abc".into()),
            name: "Marketer name".into(),
        }
    }

    /// Marketers are market makers. They pull the supply from the sellers, and put it on the market.
    fn makes_market(self, supply: Supply) -> Option<Ad> {
        if supply.has_supply_available() == false {
            return None;
        }

        // FIXME: make the state transition to be exectued
        // supply.state = SupplyState::Marketed;

        Some(Ad::new(self.id, supply))
    }
}

#[derive(Debug,Hash,PartialEq,Eq)]
struct BuyerId(String);
struct Buyer {
    id: BuyerId,
    name: String,
}

impl Buyer {
    fn new(name: String) -> Self {
        Buyer {
            id: BuyerId("abc".into()),
            name,
        }
    }

    fn bids(self, ad: Ad) -> Transaction {
        // FIXME: update the state of the underlying supply
        // ad.supply.state = SupplyState::Consumed;

        Transaction::new(ad, self.id)
    }
}

struct Supply {
    available_items: u32,
    provided_by: SellerId,
    state: SupplyState,
}

impl Supply {
    fn new(seller_id: SellerId, available_items: u32) -> Self {
        Self {
            provided_by: seller_id,
            state: SupplyState::Created,
            available_items,
        }
    }

    fn has_supply_available(&self) -> bool {
        self.available_items > 0
    }
}

enum SupplyState {
    Created,
    Marketed,
    Consumed,
}

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

    fn get_maker(self, market: Market) -> Option<Marketer> {
        market.marketers.get(
            &self.ad.marketer
        )
    }

    fn get_taker(&self, market: Market) -> Option<Buyer> {
        market.buyers.get(&self.taker)
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
        let makret = Market::new();

        // actors
        let seller = Seller::new();
        let marketer = Marketer::new();
        let buyer = Buyer::new("mr buyer".into());

        // available supply must be defined so the market maker doesn't offer anything
        // out of thing air
        let ads_listing: Vec<Ad> = (0..9).into_iter().filter_map(|i| {
            // first, a seller needs to create supply
            let supply = seller.creates_supply(i);
            // later, a marketer will pick up this supply and put it on the market
            // FIXME: borrowing error
            marketer.makes_market(supply)
        }).collect();

        // let's use some randomness!
        let mut rng = thread_rng();

        // once supply has been put on the market, it is now advertisment, or in short: an ad
        // the ad can be bid against by a buyer, which in turn creates a transaction
        // between the market maker (the marketer) and the market taker (buyer)
        if let Some(ad) = ads_listing.choose(&mut rng) {
            let transaction = buyer.bids(*ad);
            println!("Tx: {:?}", transaction);
        }
    }
}
