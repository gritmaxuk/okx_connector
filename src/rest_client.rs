use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Orderbook {
    asks: Vec<(f64, f64)>,
    bids: Vec<(f64, f64)>,
}

impl Orderbook {
    pub fn from_snapshot(data: &str) -> Result<Self, Box<dyn Error>> {
        let response: OrderbookResponse = serde_json::from_str(data)?;
        let orderbook = response.data.into_iter().next().ok_or("Empty response data")?;
        Ok(orderbook)
    }

    pub fn apply_update(&mut self, update: &str) {
        // Logic to apply updates to the order book
        let update: Orderbook = serde_json::from_str(update).unwrap();
        self.asks.extend(update.asks);
        self.bids.extend(update.bids);
        self.sort_order_book();
    }

    fn sort_order_book(&mut self) {
        // Sort asks in ascending order and bids in descending order
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

pub struct OKXRestClient {
    base_url: String,
    client: Client,
}

impl OKXRestClient {
    pub fn new(base_url: &str) -> Self {
        OKXRestClient {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn get_order_book(&self, symbol: &str) -> Result<Orderbook, Box<dyn Error>> {
        let url = format!("{}/api/v5/market/books?instId={}", self.base_url, symbol);
        let response = self.client.get(&url).send().await?.text().await?;
        Orderbook::from_snapshot(&response)
    }
}