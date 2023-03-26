use crate::models::asset::Asset;

#[derive(Debug)]
pub struct Price {
    pub asset: Asset,
    pub price: f64
}