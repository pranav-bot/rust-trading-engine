use std::collections::HashMap;
use super::orderbook::{Order, OrderBook};
use rust_decimal::prelude::*;

//BTCUSD
//BTC -> Base
//USE -> Quote
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair{
        TradingPair{base, quote}
    }
    pub fn to_string(self) -> String{
        format!("{}_{}", self.base, self.quote)
    }
}

pub struct MatchingEngine{
    orderbooks: HashMap<TradingPair, OrderBook>
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine{
        MatchingEngine{
            orderbooks: HashMap::new(),
        }
    }
    pub fn add_new_market(&mut self, pair: TradingPair){
        self.orderbooks.insert(pair.clone(), OrderBook::new());
        println!("Opening new orderbook for market {:?}", pair.to_string());
    }
    pub fn place_limit_order(&mut self, pair: TradingPair, price: Decimal, order: Order) -> Result<(), String>{
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);
                println!("Placed limit order at price level{:?}", price);
                Ok(())
            }
            None => {
                Err(format!("The order book for the given trading pair ({}) does not exist",
            pair.to_string()
            ))
            }
        }
    }
}