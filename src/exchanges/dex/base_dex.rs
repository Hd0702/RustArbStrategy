use async_trait::async_trait;
use crate::coins::Coin;

#[async_trait]
pub trait BaseDex {
    async fn get_price(&self, token_in: Coin, token_out: Coin, amount: u128) -> Result<u128, Box<dyn std::error::Error>>;
    fn get_name(&self) -> String;
}