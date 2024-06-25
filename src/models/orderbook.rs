use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Orderbook {
    pub asks: Vec<(f64, f64)>,
    pub bids: Vec<(f64, f64)>,
}

impl Orderbook {
    pub fn from_snapshot(data: &str) -> Result<Self, Box<dyn Error>> {
        let response: OrderbookResponse = serde_json::from_str(data)?;
        let orderbook = response.data.into_iter().next().ok_or("Empty response data")?;
        Ok(orderbook)
    }

    pub fn apply_update(&mut self, update: &str) {
        let update: Orderbook = serde_json::from_str(update).unwrap();
        self.asks.extend(update.asks);
        self.bids.extend(update.bids);
        self.sort_order_book();
    }

    fn sort_order_book(&mut self) {
        self.asks.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        self.bids.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderbookResponse {
    code: String,
    msg: String,
    data: Vec<Orderbook>,
}