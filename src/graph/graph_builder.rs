use std::collections::HashMap;
use std::error::Error;




use strum_macros::EnumIter;
use priority_queue::double_priority_queue::DoublePriorityQueue;
use crate::coins::Coin;
use crate::graph::path::{Path, PriceCall};
use crate::exchanges::{BaseDex};
use strum::IntoEnumIterator;
use crate::exchanges::dex::curve::CURVE_INSTANCE;
use crate::exchanges::dex::quickswap_v2::QUICKSWAP_V2_INSTANCE;
use crate::exchanges::dex::sushi_swap::SUSHI_INSTANCE;


pub struct GraphBuilder {
    // split by coin since there is no easy way to measure one coin against another w/o a good amount of work
     coin_queue: HashMap<Coin, DoublePriorityQueue<Path, i128>>,
     complete_graph: Vec<Path>
}

#[derive(EnumIter, PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Providers {
    Curve,
    QuickswapV2,
    Sushi
}

impl Providers {
    const VALUES: &'static [Providers] = &[Providers::Curve, Providers::QuickswapV2, Providers::Sushi];
    pub fn get_exchange(&self) -> &'static dyn BaseDex {
        match self {
            Providers::Curve => &*CURVE_INSTANCE,
            Providers::QuickswapV2 => &*QUICKSWAP_V2_INSTANCE,
            Providers::Sushi => &*SUSHI_INSTANCE
        }
    }
}

// we just need to build the graphs once, but we will constantly need to update prices
impl GraphBuilder {
    pub async fn new() -> Self {
        let mut coin_queue = HashMap::new();
        for coin in Coin::iter() {
            coin_queue.insert(coin, DoublePriorityQueue::new());
        }
        let mut item = Self {
            coin_queue,
            complete_graph: build_vertices()
        };
        item.evaluate_opportunities().await;
        item
    }
    // let's keep this simple and have one thread for execution and another for getting prices. We'll improve over time
    pub async fn evaluate_opportunities(&mut self) {
        for path in &self.complete_graph {
            let mut first = path.calls.first().expect("expected call list to not be empty").clone();
            let res = find_peak(&first).await;
            first.amount = res.unwrap();
            if first.amount > 0 {
                let mut new_path: Vec<PriceCall> = vec![first];
                for e in path.calls[1..].iter().enumerate() {
                    let (i, &call): (usize, &PriceCall) = e;
                    new_path.push(
                        PriceCall {
                            input_coin: call.input_coin,
                            output_coin: call.output_coin,
                            amount: call.exchange.get_price(call.input_coin, call.output_coin, new_path[i].amount).await.unwrap(),
                            exchange: call.exchange
                        }
                    )
                }
                let amount = if new_path.first().unwrap().amount < new_path.last().unwrap().amount {
                    new_path.last().unwrap().amount - new_path.first().unwrap().amount
                } else {
                    0
                };
                let first_coin = &new_path.first().unwrap().input_coin.clone();
                self.coin_queue.get_mut(first_coin).unwrap().push(Path { calls: new_path }, amount as i128);
            }
        }
    }
}

// there will be a price that increases until it doesn't. Find that peak
// prime candidate for unit tests
async fn find_peak(price_call: &PriceCall) -> Result<u128, Box<dyn Error>> {
    // arbitrary end. Should improve on this
    let (mut start, mut end) = (0, i64::MAX as u128);
    let (start_coin, end_coin) = (price_call.input_coin, price_call.output_coin);
    while start < end {
        let mid = start + (end - start) / 2;
        let (current, next) = (price_call.exchange.get_price(start_coin, end_coin, mid).await?, price_call.exchange.get_price(start_coin, end_coin, mid + 1).await?);
        if current < next {
            start = current + 1
        } else {
            end = current
        }
    }
    Ok(start)
}

// for exchanges we can do all permutations. It doesn't need to end at a particular exchange
// However, for coins we can do all permutations as long as the start coin and the end coin are the same
// So basically like permutations of n -1 where n = the number of exchanges for a particular exchange.
// One thing to note is that the starting coin does not count as number in N. It is just something that is unaccounted for.
fn build_vertices() -> Vec<Path> {
    // just maintains the providers. Has no idea about the coins?
    let mut internal_graph: Vec<Vec<&Providers>> = Vec::new();
    // then we will populate the next vec but with amount 0.
    // then we will constantly update the prices in the internal vec
    let mut complete_graph: Vec<Path> = Vec::new();
    fn backtrack<'a>(index: u8, end: u8, result: &mut Vec<Vec<&'a Providers>>, path: &mut Vec<&'a Providers>) {
        if index == end {
            result.push(path.clone());
            return;
        }
        if index > 1 {
            result.push(path.clone());
        }
        for provider in Providers::VALUES.iter() {
            path.push(&provider);
            backtrack(index + 1, end, result, path);
            path.pop();
        }
    }
    backtrack(0, 3, &mut internal_graph, &mut Vec::new());
    println!("{:?}", internal_graph);

    fn add_coins(output_coin: Coin, coins_available: &mut Vec<Coin>, providers_left: &mut Vec<&Providers>, result: &mut Vec<Path>, path: &mut Path) {
        if providers_left.len() == 0 {
            path.add_call(PriceCall::new((&path.calls.last()).unwrap().output_coin, output_coin, 0, path.calls.first().unwrap().exchange.clone()));
            result.push(path.clone());
            return;
        }
        let local_coins = coins_available.clone();
        let provider = providers_left.pop().unwrap();
        for coin in local_coins {
            if path.calls.len() == 0 {
                path.add_call(PriceCall::new(output_coin, coin, 0, provider.get_exchange()));
            } else {
                path.add_call(PriceCall::new((&path.calls.last()).unwrap().output_coin, coin, 0, provider.get_exchange()));
            }
            coins_available.remove(coins_available.iter().position(|x| *x == coin).unwrap());
            add_coins(output_coin, coins_available, providers_left, result, path);
            coins_available.push(coin);
        }
    }
    for coin in Coin::iter() {
        let mut coins_available = Coin::iter().filter(|x| *x != coin).collect::<Vec<Coin>>();
        for route in &internal_graph {
            let mut path = Path::new();
            add_coins(coin, &mut coins_available, &mut route.clone(), &mut complete_graph, &mut path);
        }
    }
    complete_graph.sort_by(| a, b | a.calls.len().cmp(&b.calls.len()));
    println!("{:?}", complete_graph);
    return complete_graph;
}