use marketplace;
/// This program enables sellers to put their offers on a marketplace,
/// and also make it possible for buyers to purcharse a product/service on this marketpalce.
fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("Hello, Mr {:?}, greetings from version: {:?}", args, marketplace::version());
}
