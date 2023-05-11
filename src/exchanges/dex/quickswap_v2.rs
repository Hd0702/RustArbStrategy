use std::error::Error;
use std::str::FromStr;
use ethers::prelude::{Address, U256};
use ethers::providers::{Http, Provider};
use once_cell::sync::Lazy;
use crate::coins::Coin;
use crate::exchanges::dex::{PROVIDER, UNISWAP_V2_ROUTER, UniswapV2Base};

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

    pub async fn get_price(&self, token_in: Coin, token_out: Coin, amount: u128) -> Result<u128, Box<dyn std::error::Error>> {
        self.uniswap_v2_base.get_price(token_in, token_out, amount).await
    }
}