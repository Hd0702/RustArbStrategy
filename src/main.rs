#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

use dotenv::dotenv;
mod exchanges;
mod models;

// up next let's turn this into a response?
// we need to do CEX arbitrage first.
// WE're just doing ETH for now.
// we need to get the price of ETH on both exchanges
// buy on one and sell on another
// this means we need to be able to buy and sell on both exchanges
// Then we can move onto DEX

// Next steps
// 1. Kraken client
// 2. Turn get price of eth API into a shared response model
// 3. Turn buy API into a shared response model
// 4. set up arb loop that looks for arb opportunities and the price if we were to buy with fees
// 5. Expand to dex arbitrage
fn main() {
    dotenv().ok();
    let coinbase = exchanges::CoinbaseClient {
        api_key: dotenv!("COINBASE_API_KEY").to_string(),
        api_secret: dotenv!("COINBASE_API_SECRET").to_string()
    };
    coinbase.hello_world();
    let kraken = exchanges::KrakenClient {
        api_key: dotenv!("KRAKEN_API_KEY").to_string(),
        api_secret: dotenv!("KRAKEN_API_SECRET").to_string()
    };
    let result = kraken.get_price(vec!["ETHUSDT"]).unwrap();
    println!("Result: {:?}", result);
}
