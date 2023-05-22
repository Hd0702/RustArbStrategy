
use std::error::Error;

use ethers::prelude::{Provider};
use ethers::providers::Http;
use ethers::types::{Address};
use once_cell::sync::Lazy;
use async_trait::async_trait;
use crate::coins::Coin;
use crate::exchanges::BaseDex;
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
}

#[async_trait]
impl BaseDex for UniswapV3Client {
    async fn get_price(&self, _token_in: Coin, _token_out: Coin, _amount: u128) -> Result<u128, Box<dyn Error>> {
        todo!("implement uniswap v3 client")
    }
    fn get_name(&self) -> String {
        "UniswapV3Client".to_string()
    }
}
