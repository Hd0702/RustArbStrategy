use std::collections::HashMap;
use std::string::ToString;
use std::sync::Arc;
use ethers::prelude::{abigen, Provider};
use ethers::providers::Http;
use ethers::solc::resolver::print;
use ethers::types::{Address, U256};
use once_cell::sync::Lazy;
use crate::coins::Coin;
use memoize::memoize;
use crate::exchanges::dex::{CURVE_AAVE, CURVE_TRICRYPTO3, PROVIDER};

pub struct Curve {
    pub pool_address: String
}

// Not used for now. Too complicated :(
impl Curve {
    pub fn new(pool_address: String) -> Self {
        Self {
            pool_address
        }
    }
    const TOKEN_MAP: Lazy<HashMap<&str, HashMap<Coin, i8>>> = Lazy::new(|| HashMap::from([
                ("0x92215849c439e1f8612b6646060b4e3e5ef822cc", HashMap::from([
                    (Coin::DAI, 0),
                    (Coin::USDC, 0),
                    (Coin::USDT, 0),
                    (Coin::WETH, 2),
                    (Coin::WBTC, 1)
                ]))
            ])
    );

    const WRAPPED_POOLS: Lazy<HashMap<Coin, (i8, &str)>> = Lazy::new(||
        HashMap::from([
            // double check these are correct
            (Coin::DAI, (0, "0x445fe580ef8d70ff569ab36e80c647af338db351")),
            (Coin::USDC, (1,"0x445fe580ef8d70ff569ab36e80c647af338db351")),
            (Coin::USDT, (2,"0x445fe580ef8d70ff569ab36e80c647af338db351"))
        ])
    );

    const CURVE_TRICRYPTO3_QUOTER: Lazy<CURVE_TRICRYPTO3<Provider<Http>>> = Lazy::new(|| {
        let pool_address: Address = "0x92215849c439e1f8612b6646060b4e3e5ef822cc".parse().unwrap();
        CURVE_TRICRYPTO3::new(pool_address, PROVIDER.clone())
    });

    const CURVE_AAVE_QUOTER: Lazy<CURVE_AAVE<Provider<Http>>> = Lazy::new(|| {
        let pool_address: Address = "0x445fe580ef8d70ff569ab36e80c647af338db351".parse().unwrap();
       CURVE_AAVE::new(pool_address, PROVIDER.clone())
    });

    // We should go one token at a time to keep things simple
    // assume we just have one pool for now
    pub async fn get_price_tricrypto3(&self, token_in: Coin, token_out: Coin, amount: u128) -> Result<u128, Box<dyn std::error::Error>> {
        if token_in.isStable() && token_out.isStable() {
            return self.get_price_aave(token_in, token_out, amount).await;
        }
        let token_in_address = Self::TOKEN_MAP.get(&*self.pool_address).ok_or("token in address not found")?.get(&token_in).ok_or("coin not found in map")?.clone();
        let token_out_address = Self::TOKEN_MAP.get(&*self.pool_address).ok_or("token out address not found")?.get(&token_out).ok_or("coin not found in map")?.clone();
        let amount = U256::from(amount);
        let fee = Self::CURVE_TRICRYPTO3_QUOTER.get_virtual_price().call().await?;

        let mut dy =  Self::CURVE_TRICRYPTO3_QUOTER.get_dy(U256::from(token_in_address), U256::from(token_out_address.clone()), U256::from(amount)).call().await?;
        // stables are LP tokens from this pool https://curve.fi/#/polygon/pools/aave/deposit wrapped in aave. Can be unrwapped by passing in _use_underlying in the contract
        if token_out_address == 0 {
            dy = Self::CURVE_AAVE_QUOTER.calc_withdraw_one_coin(dy, Self::WRAPPED_POOLS.get(&token_out).unwrap().0 as i128).call().await?;
        }
        println!(" dy is {dy:?} and fee is {fee:?}");
        Ok(dy.as_u128())
    }

    pub async fn get_price_aave(&self, token_in: Coin, token_out: Coin, amount: u128) -> Result<u128, Box<dyn std::error::Error>> {
        let amount = U256::from(amount);
        let fee = Self::CURVE_AAVE_QUOTER.get_virtual_price().call().await?;
        let token_in_id = Self::WRAPPED_POOLS.get(&token_in).ok_or("aave token in address not found")?.0 as i128;
        let token_out_id = Self::WRAPPED_POOLS.get(&token_out).ok_or("aave token out address not found")?.0 as i128;
        let dy = Self::CURVE_AAVE_QUOTER.get_dy_underlying(token_in_id, token_out_id, amount).call().await?;
        println!("aave dy is {dy:?} and fee is {fee:?}");
        // now let's calc with another version
        // let mut amounts = [U256::from(0); 3];
        // amounts[token_in_id as usize] = amount;
        // let lp_token_amount = Self::CURVE_AAVE_QUOTER.calc_token_amount(amounts, true).call().await?;
        // let dy_second = Self::CURVE_AAVE_QUOTER.calc_withdraw_one_coin(lp_token_amount, token_out_id).call().await?;
        // println!("Second dy is {dy_second:?}");
        Ok(dy.as_u128())
    }
}