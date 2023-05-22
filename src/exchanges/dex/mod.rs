pub mod uniswap_v3_client;
pub(crate) mod curve;
pub(crate) mod sushi_swap;
pub(crate) mod quickswap_v2;
mod uniswap_v2_base;
mod base_dex;

use std::sync::Arc;
use ethers::prelude::{abigen, Http, Provider};
use once_cell::sync::Lazy;
pub use uniswap_v3_client::UniswapV3Client;
pub use curve::Curve;
pub use sushi_swap::Sushi;
pub use quickswap_v2::QuickswapV2;
pub use base_dex::BaseDex;
use uniswap_v2_base::UniswapV2Base;

abigen!(CURVE_TRICRYPTO3,"./src/exchanges/dex/contracts/curve_quoter.json");
abigen!(CURVE_AAVE, "./src/exchanges/dex/contracts/curve_aave.json");
abigen!(UNISWAP_V2_ROUTER, "./src/exchanges/dex/contracts/uniswap_v2_router.json");
abigen!(UNISWAP_V3_ROUTER, "./src/exchanges/dex/contracts/uniswap_v3_quoter.json");
pub use CURVE_AAVE;
pub use CURVE_TRICRYPTO3;
pub use UNISWAP_V2_ROUTER;
pub use UNISWAP_V3_ROUTER;

pub const PROVIDER: Lazy<Arc<Provider<Http>>> = Lazy::new(|| {
    Arc::new(Provider::<Http>::try_from("https://polygon-mainnet.g.alchemy.com/v2/CCkZPlb6eku3NMM5jFNWZ7tvGHrlOUtJ").unwrap())
});
