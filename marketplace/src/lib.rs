use std::fmt::Debug;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Seller<'a> {
    id: &'a str,
    name: &'a str,
}

impl<'a> Seller<'a> {
    fn new() -> Self {
        Self {
            id: &"abc",
            name: &"seller name",
        }
    }

    fn creates_supply(&self, available_items: u32) -> Supply {
        Supply::new(&self, available_items)
    }
}

#[derive(Debug)]
struct Supply<'a> {
    available_items: u32,
    provided_by: &'a Seller<'a>,
    state: SupplyState,
}

#[derive(Debug)]
enum SupplyState {
    Created,
    Marketed,
    Consumed,
}

impl<'a> Supply<'a> {
    fn new(seller: &'a Seller, available_items: u32) -> Self {
        Self {
            provided_by: seller,
            state: SupplyState::Created,
            available_items,
        }
    }

    fn has_supply_available(&self) -> bool {
        self.available_items > 0
    }
}

#[derive(Debug)]
struct Marketer<'a> {
    id: &'a str,
    name: &'a str,
}

impl<'a> Marketer<'a> {
    fn new() -> Self {
        Marketer {
            id: &"abc",
            name: &"Marketer name",
        }
    }

    /// Marketers are market makers. They pull the supply from the sellers, and put it on the market.
    fn makes_market(&self, supply: &'a mut Supply) -> Option<Ad> {
        if supply.has_supply_available() == false {
            return None;
        }

        // FIXME: make the state transition to be exectued
        supply.state = SupplyState::Marketed;

        Some(Ad::new(&self, supply))
    }
}

#[derive(Debug)]
struct Ad<'a> {
    marketer: &'a Marketer<'a>,
    supply: &'a Supply<'a>,
}

impl<'a> Ad<'a> {
    fn new(marketer: &'a Marketer<'a>, supply: &'a Supply) -> Self {
        Self {
            marketer,
            supply,
        }
    }
}

#[derive(Debug)]
struct Transaction<'a> {
    ad: &'a Ad<'a>,
    taker: &'a Buyer<'a>,
}

impl<'a> Transaction<'a> {
    fn new(ad: &'a Ad, taker: &'a Buyer) -> Self {
        Self {
            ad,
            taker,
        }
    }

    fn get_maker(&self) -> &'a Marketer {
        self.ad.marketer
    }

    fn get_taker(&self) -> &'a Buyer {
        self.taker
    }
}

#[derive(Debug)]
struct Buyer<'a> {
    id: &'a str,
    name: &'a str,
}

impl<'a> Buyer<'a> {
    fn new(name: &'a str) -> Self {
        Buyer {
            id: &"abc",
            name,
        }
    }

    fn bids(&self, ad: &'a Ad<'a>) -> Transaction {
        // FIXME: update the state of the underlying supply
        // ad.supply.state = SupplyState::Consumed;

        Transaction::new(ad, &self)
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
        // actors
        let seller = Seller::new();
        let marketer = Marketer::new();
        let buyer = Buyer::new(&"mr buyer");

        // available supply must be defined so the market maker doesn't offer anything
        // out of thing air
        let ads_listing: Vec<Ad> = (0..9).into_iter().filter_map(|i| {
            // first, a seller needs to create supply
            let mut supply = seller.creates_supply(i);
            // later, a marketer will pick up this supply and put it on the market
            // FIXME: borrowing error
            marketer.makes_market(&mut supply)
        }).collect();

        // let's use some randomness!
        let mut rng = thread_rng();

        // once supply has been put on the market, it is now advertisment, or in short: an ad
        // the ad can be bid against by a buyer, which in turn creates a transaction
        // between the market maker (the marketer) and the market taker (buyer)
        if let Some(ad) = ads_listing.choose(&mut rng) {
            let transaction = buyer.bids(ad);
            println!("Tx: {:?}", transaction);
        }
    }
}
