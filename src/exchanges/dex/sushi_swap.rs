use std::error::Error;

use ethers::prelude::{Address};

use futures::executor::block_on;
use once_cell::sync::Lazy;
use async_trait::async_trait;
use crate::coins::Coin;
use crate::exchanges::BaseDex;
use crate::exchanges::dex::{UniswapV2Base};

pub struct Sushi {
    uniswap_v2_base: UniswapV2Base
}

impl Sushi {
    pub fn new() -> Self {
        let router_address: Address = "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506".parse().unwrap();
        Self {
            uniswap_v2_base: UniswapV2Base::new(router_address)
        }
    }
}

#[async_trait]
impl BaseDex for Sushi {
    async fn get_price(&self, token_in: Coin, token_out: Coin, amount: u128) -> Result<u128, Box<dyn Error>> {
        block_on(self.uniswap_v2_base.get_price(token_in, token_out, amount))
    }
    fn get_name(&self) -> String {
        "Sushi".to_string()
    }
}

pub static SUSHI_INSTANCE: Lazy<Sushi> = Lazy::new(|| {
    Sushi::new()
});