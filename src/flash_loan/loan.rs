use std::sync::Arc;
use ethers::abi::Address;
use ethers::prelude::{abigen, Http, Middleware, Provider};

pub struct Loan {}
const LOAN_ADDRESS: &str = "0x33d8d437796bd43bdccc6740c585f4a15d1070b7";
const WETH_USDT_POOL: &str = "0x4ccd010148379ea531d6c587cfdd60180196f9b1";
impl Loan {
    pub async fn get_gas(&self) -> u64 {
        let provider = Arc::new(Provider::<Http>::try_from("https://polygon-mainnet.g.alchemy.com/v2/CCkZPlb6eku3NMM5jFNWZ7tvGHrlOUtJ").unwrap());
        let address: Address = LOAN_ADDRESS.parse().unwrap();
        provider.get_gas_price().await.unwrap().as_u64()
    }
}