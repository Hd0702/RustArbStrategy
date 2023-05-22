use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Coin {
    USDT,
    USDC,
    DAI,
    WETH,
    WBTC
}

impl Coin {
    pub fn address(&self) -> &str {
        match *self {
            Coin::USDT => "0xc2132d05d31c914a87c6611c10748aeb04b58e8f",
            Coin::USDC => "0x2791bca1f2de4661ed88a30c99a7a9449aa84174",
            Coin::DAI => "0x8f3cf7ad23cd3cadbd9735aff958023239c6a063",
            Coin::WETH => "0x7ceb23fd6bc0add59e62ac25578270cff1b9f619",
            Coin::WBTC => "0x1bfd67037b42cf73acf2047067bd4f2c47d9bfd6"
        }
    }

    pub fn isStable(&self) -> bool {
        match *self {
            Coin::USDT => true,
            Coin::USDC => true,
            Coin::DAI => true,
            _ => false
        }
    }
}