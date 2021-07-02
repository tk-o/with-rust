#[derive(Debug)]
struct Supply {
    id: String,
    name: String,
    quantity: u16,
}

struct Provider {
    name: String,
    inventory: Vec<Supply>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use rand::seq::SliceRandom;
    use std::convert::TryInto;

    #[test]
    fn it_handles_basic_market_setup() {
        // first, we need some supply-side on the market

        // the providers will take care of manufacturing goods and services
        let mut provider_4f = Provider {
            name: "4f".into(),
            inventory: vec![
                Supply {
                    id: "4f-1".into(),
                    name: "Winter jacket".into(),
                    quantity: 100,
                },
            ],
        };
        let provider_nike = Provider {
            name: "nike".into(),
            inventory: vec![
                Supply {
                    id: "ni-1".into(),
                    name: "Football shoes - pair".into(),
                    quantity: 100,
                },
                Supply {
                    id: "ni-2".into(),
                    name: "Football pads - pair".into(),
                    quantity: 200,
                }
            ],
        };
        let provider_adidas = Provider {
            name: "adidas".into(),
            inventory: vec![
                Supply {
                    id: "ad-1".into(),
                    name: "Tracksuite top".into(),
                    quantity: 50,
                },
                Supply {
                    id: "ad-2".into(),
                    name: "Tracksuite bottom".into(),
                    quantity: 40,
                },
                Supply {
                    id: "ad-3".into(),
                    name: "Socks - pair".into(),
                    quantity: 444,
                },
            ],
        };

        // Skills-check: let's see if I understand borrow checker better
        &provider_4f.inventory.push(Supply {
            id: "4f-2".into(),
            name: "Snowboard jacket".into(),
            quantity: 20,
        });

        let mut all_providers = HashMap::new();
        all_providers.insert(&provider_4f.name, &provider_4f);
        all_providers.insert(&provider_nike.name, &provider_nike);
        all_providers.insert(&provider_adidas.name, &provider_adidas);

        // I'd like to push some new items to the providers' inventory here
        // Borrow checker won't let me do that, as I am mixing borrow with borrow-mut calls
        // Mutable borrows must occur first, and then they must be followed by regular borrows only

        assert_eq!(&provider_4f.inventory.len(), &usize::from(2usize));
        assert_eq!(&all_providers.get(&provider_4f.name).unwrap().inventory.len(), &usize::from(2usize));

        // the markerters will make sure the supply-side is easily available to the demand-side
        let marketer = ();

        let aggregated_inventory: Vec<&Supply> = all_providers.into_iter()
            .flat_map(|(k, v)| v.inventory.iter())
            .collect();

        // TODO: use choose_multiple to pick random items to market
        // let mut rng =rand::thread_rng();
        let choice: &[&Supply] = &aggregated_inventory[2..=5];

        println!("Item picked for marketing: {:?}", choice);
    }
}
