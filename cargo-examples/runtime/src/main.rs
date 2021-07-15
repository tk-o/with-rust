use erc_20::Erc20;
use erc_1155::Erc1155;

fn main() {
    println!(
        "Hello, world! {} and {}",
        Erc20::new().name(),
        Erc1155::new().name()
    );
}
