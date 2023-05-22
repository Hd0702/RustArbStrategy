

use ethers::prelude::{Address, U256};
use ethers::providers::{Http, Provider};

use crate::coins::Coin;
use crate::exchanges::dex::{PROVIDER, UNISWAP_V2_ROUTER};

pub(crate) struct UniswapV2Base {
    router: UNISWAP_V2_ROUTER<Provider<Http>>
}

impl UniswapV2Base {
    pub fn new(router_address: Address) -> Self {
        let cloned_provider = PROVIDER.clone();
        let owned_address = router_address.clone();
        Self {
            router: UNISWAP_V2_ROUTER::new(owned_address, cloned_provider)
        }
    }

    pub async fn get_price(&self, token_in: Coin, token_out: Coin, amount: u128) -> Result<u128, Box<dyn std::error::Error>> {
        let address_in: Address = token_in.address().parse()?;
        let address_out: Address = token_out.address().parse()?;
        let price = self.router.get_amounts_out(U256::from(amount), vec![address_in, address_out]).await?;
        println!("uniswap base price: {:?}", price);
        Ok(price[1].as_u128())
    }
}