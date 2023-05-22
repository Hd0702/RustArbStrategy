use std::error::Error;

use ethers::prelude::{Address};

use once_cell::sync::Lazy;
use async_trait::async_trait;
use crate::coins::Coin;
use crate::exchanges::BaseDex;
use crate::exchanges::dex::{UniswapV2Base};

pub struct QuickswapV2 {
    uniswap_v2_base: UniswapV2Base
}

impl QuickswapV2 {
    pub fn new() -> Self {
        let router_address: Address = "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff".parse().unwrap();
        Self {
            uniswap_v2_base: UniswapV2Base::new(router_address)
        }
    }
}

#[async_trait]
impl BaseDex for QuickswapV2 {
    async fn get_price(&self, token_in: Coin, token_out: Coin, amount: u128) -> Result<u128, Box<dyn Error>> {
        self.uniswap_v2_base.get_price(token_in, token_out, amount).await
    }
    fn get_name(&self) -> String {
        "QuickswapV2".to_string()
    }
}

pub static QUICKSWAP_V2_INSTANCE: Lazy<QuickswapV2> = Lazy::new(|| {
    QuickswapV2::new()
});