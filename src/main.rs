extern crate dotenv_codegen;
extern crate dotenv;

use dotenv::dotenv;
use crate::coins::Coin;
use crate::exchanges::{Curve, Sushi, QuickswapV2, BaseDex};
use crate::graph::GraphBuilder;

mod exchanges;
mod models;
mod utils;
mod coins;
mod graph;

// ideas
// we need to associate by pools, not by tokens
// each pool has a map of different tokens
// something like calc_token_amount tells us the gas price
// well need to build an adjacency list of tokens and the tokens they can go to
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let _graph = GraphBuilder::new().await;
    Sushi::new().get_price(Coin::USDT, Coin::USDC, 100000).await.unwrap();
    QuickswapV2::new().get_price(Coin::USDT, Coin::USDC, 100000).await.unwrap();
    Curve::new("0x92215849c439e1f8612b6646060b4e3e5ef822cc".to_string())
        .get_price(Coin::USDT, Coin::USDC, 100000).await.unwrap();
    Ok(())
}
