use std::hash::Hash;

use crate::coins::Coin;
use crate::exchanges::BaseDex;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Path {
    pub calls: Vec<PriceCall>
}

impl Path {
    pub fn new() -> Self {
        Self {
            calls: Vec::new()
        }
    }

    pub fn add_call(&mut self, call: PriceCall) {
        self.calls.push(call);
    }
}

#[derive(Clone)]
pub struct PriceCall {
    pub input_coin: Coin,
    pub output_coin: Coin,
    pub amount: u128,
    pub exchange: &'static dyn BaseDex
}

impl PriceCall {
    pub fn new(input_coin: Coin, output_coin: Coin, amount: u128, exchange: &'static dyn BaseDex) -> Self {
        Self {
            input_coin,
            output_coin,
            amount,
            exchange
        }
    }
}

impl std::fmt::Debug for PriceCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} -> {:?} @ {} w/ {}", self.input_coin, self.output_coin, self.amount, self.exchange.get_name())
    }
}


impl PartialEq for PriceCall {
    fn eq(&self, other: &Self) -> bool {
        self.input_coin == other.input_coin && self.output_coin == other.output_coin && self.amount == other.amount
    }
}

impl Eq for PriceCall {
}

impl Hash for PriceCall {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.input_coin.hash(state);
        self.output_coin.hash(state);
        self.amount.hash(state)
    }
}