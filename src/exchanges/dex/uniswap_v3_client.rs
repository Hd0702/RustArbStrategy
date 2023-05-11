use std::collections::HashMap;
use std::sync::Arc;
use ethers::prelude::{abigen, Provider};
use ethers::providers::Http;
use ethers::types::{Address, U256};
use once_cell::sync::Lazy;
use crate::coins::Coin;
use crate::exchanges::dex::{PROVIDER, UNISWAP_V3_ROUTER};

pub struct UniswapV3Client{
}


impl UniswapV3Client {
    // currently hardcoding the token
    pub fn new() -> Self {
        Self {}
    }

    const QUOTER: Lazy<UNISWAP_V3_ROUTER<Provider<Http>>> = Lazy::new(|| {
        let pool_address: Address = "0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6".parse().unwrap();
        UNISWAP_V3_ROUTER::new(pool_address, PROVIDER.clone())
    });

    // we also need to figure out the fee as well. This is different for every pool. Will need to do an approach like curve
    pub async fn get_price(&self, token_in: Coin, token_out: Coin, amount: u128) -> Result<u128, Box<dyn std::error::Error>> {
        panic!("not implemented")
    }
}
